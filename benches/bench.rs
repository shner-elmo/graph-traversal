use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graphdb::Data;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn criterion_benchmark(c: &mut Criterion) {
    let file = File::open("data/parent_children_map.json").unwrap();
    let reader = BufReader::new(file);
    let hashmap = serde_json::from_reader::<_, HashMap<String, Vec<String>>>(reader).unwrap();
    let data = Data::from_iter(hashmap.iter().map(|(k, v)| {
        (
            k.as_str(),
            v.into_iter().map(|x| x.as_str()).collect::<Vec<_>>(),
        )
    }));

    // c.bench_function("instantiate db", |b| b.iter(|| Data::from_iter(
    //     hashmap.iter()
    //         .map(|(k, v)| (k.as_str(), v.into_iter().map(|x| x.as_str()).collect::<Vec<_>>()))
    // )));  // 1.0756 s

    c.bench_function("get children", |b| {
        b.iter(|| data.get_children(black_box(&"Enciclopedia")))
    });
    c.bench_function("get children", |b| {
        b.iter(|| data.get_children(black_box(&"Categorie")))
    });
    c.bench_function("get children", |b| {
        b.iter(|| data.get_children(black_box(&"Categoria")))
    });
    c.bench_function("get children", |b| {
        b.iter(|| data.get_children(black_box(&"Imperatori ROmani")))
    });
    c.bench_function("get children", |b| {
        b.iter(|| data.get_children(black_box(&"Imperatori Romani")))
    });
    c.bench_function("get children", |b| {
        b.iter(|| data.get_children(black_box(&"Imperatori Romani")))
    });

    c.bench_function("find descendants count", |b| {
        b.iter(|| data.descendants_iter([black_box("Categorie")]).count())
    });
    c.bench_function("find descendants count", |b| {
        b.iter(|| data.descendants_iter([black_box("Enciclopedia")]).count())
    });
    c.bench_function("find descendants count", |b| {
        b.iter(|| data.descendants_iter([black_box("Liste")]).count())
    });
    c.bench_function("find descendants count", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Imperatori_romani")])
                .count()
        })
    });
    c.bench_function("find descendants count", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Gaio_Giulio_Cesare")])
                .count()
        })
    });
    c.bench_function("find descendants count", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Alessandro_Manzoni")])
                .count()
        })
    });

    c.bench_function("find descendants last", |b| {
        b.iter(|| data.descendants_iter([black_box("Categorie")]).last())
    });
    c.bench_function("find descendants last", |b| {
        b.iter(|| data.descendants_iter([black_box("Enciclopedia")]).last())
    });
    c.bench_function("find descendants last", |b| {
        b.iter(|| data.descendants_iter([black_box("Liste")]).last())
    });
    c.bench_function("find descendants last", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Imperatori_romani")])
                .last()
        })
    });
    c.bench_function("find descendants last", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Gaio_Giulio_Cesare")])
                .last()
        })
    });
    c.bench_function("find descendants last", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Alessandro_Manzoni")])
                .last()
        })
    });

    c.bench_function("find descendants to vec", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Categorie")])
                .collect::<Vec<_>>()
        })
    });
    c.bench_function("find descendants to vec", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Enciclopedia")])
                .collect::<Vec<_>>()
        })
    });
    c.bench_function("find descendants to vec", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Liste")])
                .collect::<Vec<_>>()
        })
    });
    c.bench_function("find descendants to vec", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Imperatori_romani")])
                .collect::<Vec<_>>()
        })
    });
    c.bench_function("find descendants to vec", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Gaio_Giulio_Cesare")])
                .collect::<Vec<_>>()
        })
    });
    c.bench_function("find descendants to vec", |b| {
        b.iter(|| {
            data.descendants_iter([black_box("Alessandro_Manzoni")])
                .collect::<Vec<_>>()
        })
    });

    c.bench_function("find descendants non-existent", |b| {
        b.iter(|| data.descendants_iter([black_box("Castegorie")]))
    });
    c.bench_function("find descendants non-existent", |b| {
        b.iter(|| data.descendants_iter([black_box("Encisclopedia")]))
    });
    c.bench_function("find descendants non-existent", |b| {
        b.iter(|| data.descendants_iter([black_box("Lisste")]))
    });
    c.bench_function("find descendants non-existent", |b| {
        b.iter(|| data.descendants_iter([black_box("Impferatori Romani")]))
    });
    c.bench_function("find descendants non-existent", |b| {
        b.iter(|| data.descendants_iter([black_box("Imperaftori")]))
    });
    c.bench_function("find descendants non-existent", |b| {
        b.iter(|| data.descendants_iter([black_box("Gaio sGiulio Cesare")]))
    });
    c.bench_function("find descendants non-existent", |b| {
        b.iter(|| data.descendants_iter([black_box("Alessandfro Manzoni")]))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
