use rust_rs::rust;

rust! {
    fk glavno() {
        neka x = 5;
        pišiLiniju!("Latinica radi: {}", x);

        neka rezultat: Rezultat<i32, Niz> = Ok(42);
        uporedi rezultat {
            Ok(br) => pišiLiniju!("Broj: {}", br),
            Ups(_) => pišiLiniju!("Greška"),
        }
    }
}
