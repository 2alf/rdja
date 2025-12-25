use rust_dk::rust;

rust! {
    // Test ћирилица
    фк главно() {
        нека x = 5;
        пишиЛинију!("Ћирилица: {}", x);

        нека резултат: Резултат<i32, Низ> = Ок(42);
        упореди резултат {
            Ок(бр) => пишиЛинију!("Број: {}", бр),
            Упс(е) => пишиЛинију!("Грешка: {}", е),
        }
    }
}

rust! {
    // Test latinica
    fk test_latin() {
        neka y = 10;
        pišiLiniju!("Latinica: {}", y);

        neka opcija: Opcija<i32> = Neke(100);
        uporedi opcija {
            Neke(vrednost) => pišiLiniju!("Vrednost: {}", vrednost),
            Nijedno => pišiLiniju!("Nema vrednosti"),
        }
    }
}

rust! {
    // Test mućkalica
    фк главно() {
        neka z = 3;
        пишиЛинију!("MEŠAJJJ БАТОО: {}", z);

        нека резултат: Резултат<i32, Низ> = Ок(42);
        упореди резултат {
            Ок(бр) => pišiLiniju!("Broj je: {}", бр),
            Упс(_) => пишиЛинију!("Грешка"),
        }
    }
}
