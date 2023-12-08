use std::borrow::Borrow;
use std::cmp::Eq;
use std::hash::Hash;
use std::iter::FusedIterator;
use std::usize;
use std::hash::BuildHasherDefault;

use indexmap::IndexSet;
use rustc_hash::{FxHasher, FxHashMap  as HashMap};

type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<FxHasher>>;

/// The Id refers to how a node or relationship is stored internally.
// The Id is actually the index at which the node lives inside the `buf` array.
pub type NodeId = u32; // `NodeId` is casted to `usize` for indexing the `buf` (and back ...)

pub struct Data<T: Eq + Hash + Clone> {
    label_id_map: HashMap<T, NodeId>, // use `str` instead of `String` so we can query the map without needing to construct a `&String` each time.
    id_label_map: HashMap<NodeId, T>,
    buf: Vec<NodeId>,
}

impl<'a, T: 'a + Eq + Hash + Clone> Data<T> {
    pub fn get_n_nodes(&self) -> usize {
        self.label_id_map.len()
    }

    /// panics if the parameter `node_id` is incorrect
    #[inline]
    fn get_children_as_ids(&self, node_id: NodeId) -> &[NodeId] {
        let node_id = usize::try_from(node_id).unwrap();
        match self.buf[node_id] {
            0 => &[],
            n_children => {
                let offset = node_id + 1;
                &self.buf[offset..offset + usize::try_from(n_children).unwrap()]
            }
        }
    }

    #[inline]
    pub fn get_children<Q: ?Sized>(&self, node: &Q) -> Option<Vec<&T>>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        let &node_id = self.label_id_map.get(node)?;

        let vec = self
            .get_children_as_ids(node_id)
            .iter()
            .map(|id| &self.id_label_map[id])
            .collect();
        Some(vec)
    }

    // the `'fc` lifetime means that the reference needs to last as long as the function call does
    pub fn descendants_iter<'fc, Q: ?Sized + 'fc>(
        &self,
        nodes: impl IntoIterator<Item = &'fc Q>,
    ) -> impl Iterator<Item = &T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        let nodes = nodes
            .into_iter()
            .map(|node| self.label_id_map.get(node))
            .flatten()
            .cloned()
            .collect();
        LazyBFS::new(nodes, &self).map(|node_id| &self.id_label_map[&node_id])
    }
}

impl<T: Eq + Hash + Clone> FromIterator<(T, Vec<T>)> for Data<T> {
    fn from_iter<I: IntoIterator<Item = (T, Vec<T>)>>(iter: I) -> Self {
        Self::from_iter(
            iter.into_iter()
                .map(|(parent, children)| {
                    children
                        .into_iter()
                        .map(move |child| (parent.clone(), child))
                })
                .flatten(),
        )
    }
}

impl<T: Eq + Hash + Clone> FromIterator<(T, T)> for Data<T> {
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        let mut parent_children_map: HashMap<T, Vec<T>> = HashMap::default();
        for (parent, child) in iter {
            let children = parent_children_map.entry(parent).or_insert(vec![]);
            children.push(child.clone());

            // each child should also be a key (even if it doesn't have any children)
            parent_children_map.entry(child).or_insert(vec![]);
        }

        let mut label_index_map: HashMap<T, NodeId> = HashMap::default();
        label_index_map.reserve(parent_children_map.len());
        let mut cursor_idx = 0;
        for (parent, children) in &parent_children_map {
            label_index_map.insert(parent.clone(), cursor_idx);
            cursor_idx += 1 + NodeId::try_from(children.len()).unwrap();
        }

        let mut buf: Vec<NodeId> = Vec::with_capacity(usize::try_from(cursor_idx).unwrap());
        for children in parent_children_map.into_values() {
            buf.push(NodeId::try_from(children.len()).unwrap());

            if !children.is_empty() {
                buf.extend(children.iter().map(|child| label_index_map[child]))
            }
        }

        let id_label_map: HashMap<NodeId, T> = label_index_map
            .iter()
            .map(|(label, &id)| (id, label.clone()))
            .collect();

        Self {
            label_id_map: label_index_map,
            id_label_map,
            buf,
        }
    }
}

struct LazyBFS<'a, T: Eq + Hash + Clone> {
    seen: FxIndexSet<NodeId>,
    data: &'a Data<T>, // needed for `get_children_as_ids` and `id_label_map`
    idx: usize,
    children_idx: usize,
    children_idx_max: usize,
}

impl<'a, T: Eq + Hash + Clone> LazyBFS<'a, T> {
    fn new(start_nodes: Vec<NodeId>, data: &'a Data<T>) -> Self {
        let seen = FxIndexSet::from_iter(start_nodes);

        Self {
            seen,
            data,
            idx: 0,
            children_idx: 0,
            children_idx_max: 0,
        }
    }
}

impl<T: Eq + Hash + Clone> LazyBFS<'_, T> {
    #[inline]
    fn find_next_child(&mut self) -> Option<NodeId> {
        while self.children_idx != self.children_idx_max {
            let node_id = self.data.buf[self.children_idx];
            self.children_idx += 1;

            if self.seen.insert(node_id) {
                return Some(node_id);
            }
        }
        None
    }
}

impl<T: Eq + Hash + Clone> Iterator for LazyBFS<'_, T> {
    type Item = NodeId;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if let child @ Some(_) = self.find_next_child() {
            return child;
        }

        // find at least one child that we haven't seen before, and return that
        while let Some(&node) = self.seen.get_index(self.idx) {
            self.idx += 1;

            let node_id = usize::try_from(node).unwrap();
            let n_children = usize::try_from(self.data.buf[node_id]).unwrap();
            if n_children != 0 {
                let offset = node_id + 1;
                self.children_idx = offset;
                self.children_idx_max = offset + n_children;

                if let child @ Some(_) = self.find_next_child() {
                    return child;
                }
            }
        }
        return None

        // loop {
            // if self.idx >= self.curr_level.len() {
            //     // if we're at the end of the current level and there is no next level; then we are finished.
            //     if self.next_level.is_empty() {
            //         return None;
            //     }
            //
            //     // self.curr_level = vec![];  // this vs `self.curr_level.clear()` ??
            //     self.curr_level.clear();
            //     std::mem::swap(&mut self.curr_level, &mut self.next_level);
            //
            //     self.level += 1;
            //     self.idx = 0;
            // }

            // let node_id = usize::try_from(self.curr_level[self.idx]).unwrap();
            // self.idx += 1;
            //
            // let n_children = usize::try_from(self.data.buf[node_id]).unwrap();
            // if n_children != 0 {
            //     let offset = node_id + 1;
            //     self.children_idx = offset;
            //     self.children_idx_max = offset + n_children;
            //
            //     if let child @ Some(_) = self.find_next_child() {
            //         return child;
            //     }
            // }
        // }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.data.label_id_map.len()))
    }
}

impl<T: Eq + Hash + Clone> FusedIterator for LazyBFS<'_, T> {}

// TODO API
//  - instantiate from `Iterator<Item = (Asref<T>, Asref<T>)>` ...

// TODO performance
//  - reorder the nodes in `Data::buf` to improve the locality (it should be constructed by doing
//    a BFS from the "root node", but also keep track of the nodes that dont fall under it)
//  - what if we sort the nodes in a given level of the BFS ... in a consistent order that will
//    make us index the `buf` array sequentially instead of randomly ?
//  - try using u32 integers for the IDs (to save memory)
//  - remove all `.clone()`
