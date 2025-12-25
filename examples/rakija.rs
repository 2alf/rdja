use rust_dk::rust;

rust! {
    користи std::io;

    struktura Igrač {
        ime: Низ,
        čaše: u32,
        pijanost: u32,
    }

    импл Igrač {
        фк novo(ime: Niz) -> Sam {
            Igrač {
                ime,
                čaše: 0,
                pijanost: 0,
            }
        }

        фк pij(&променљиво сам) {
            sam.čaše += 1;
            sam.pijanost += 15;
            пишиЛинију!("\n {} pije rakiju! (Čaša #{})", sam.ime, sam.čaše);

            ако sam.pijanost >= 100 {
                pišiLiniju!(" {} se prepio/la! Ништа.. ај на испирање!", sam.ime);
                пишиЛинију!("Popio/la si {} čašu rakije!", sam.čaše);
            } inače ako sam.pijanost >= 70 {
                pišiLiniju!(" {} jedva stoji... (Pijanost: {}%)", sam.ime, sam.pijanost);
            } иначе ako sam.pijanost >= 40 {
                пишиЛинију!(" {} se dobro oseća! (Pijanost: {}%)", sam.ime, sam.pijanost);
            } inače {
                pišiLiniju!(" {} je tek počeo/la! (Pijanost: {}%)", sam.ime, sam.pijanost);
            }
        }

        fk odmori(&promenljivo sam) {
            ako sam.pijanost >= 20 {
                sam.pijanost -= 20;
                пишиЛинију!("\nZzZZZZzzz... {} odmara malo... (Pijanost: {}%)", sam.ime, sam.pijanost);
            } inače {
                sam.pijanost = 0;
                pišiLiniju!("\n {} je potpuno trezan/na!", sam.ime);
            }
        }

        фк zamezi(&promenljivo sam) {
            ako sam.pijanost >= 10 {
                sam.pijanost -= 10;
                пишиЛинију!("\n {} jede sir i pršutu! (Pijanost: {}%)", sam.ime, sam.pijanost);
            } иначе {
                пишиЛинију!("\n {} grize nešto...", sam.ime);
            }
        }

        fk je_gotov(&sam) -> bulovo {
            sam.pijanost >= 100
        }
    }

    fk prikazi_meni() {
        pišiLiniju!("\n━━━━━━━━━━━━━━━━━━━━━━");
        pišiLiniju!("  * РАКИЈА СИМУЛАТОР *");
        pišiLiniju!("━━━━━━━━━━━━━━━━━━━━━━");
        пишиЛинију!("1. Pij rakiju ");
        pišiLiniju!("2. Odmori malo ");
        pišiLiniju!("3. Замези мало ");
        пишиЛинију!("4. Izađi iz kafane више ");
        pišiLiniju!("━━━━━━━━━━━━━━━━━━━━━━");
        pišiLiniju!("Izaberi:");
    }

    фк učitaj_izbor() -> Niz {
        neka promenljivo unos = Niz::novo();
        io::stdin()
            .read_line(&promenljivo unos)
            .očekuj("Greška pri čitanju");
        unos.trim().u_ovo()
    }

    fk главно() {
        pišiLiniju!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        пишиЛинију!("  🍾 Dobrodošao u kafanu! 🍾");
        pišiLiniju!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        пишиЛинију!("\nКако се зовеш?");

        neka ime = učitaj_izbor();
        neka променљиво igrač = Igrač::novo(ime.clone());

        pišiLiniju!("\nŽiveli! {}! Sedi, naruči!", igrač.ime);
        пишиЛинију!("  Pazi.. ako dostigneš 100% pijanosti, igra je gotova!");

        petlja {
            ako igrač.je_gotov() {
                prekini;
            }

            prikazi_meni();
            neka izbor = učitaj_izbor();

            uporedi izbor.kao_ref() {
                "1" => igrač.pij(),
                "2" => igrač.odmori(),
                "3" => igrač.zamezi(),
                "4" => {
                    pišiLiniju!("\n {} odlazi iz kafane. Popio/la {} čaša!", igrač.ime, igrač.čaše);
                    pišiLiniju!("Doviđenja!");
                    прекини;
                }
                _ => pišiLiniju!("\n Nema toga u meniju brate"),
            }
        }

        pišiLiniju!("\n━━━━━━━━━━━━━━━━━━━━━━");
        пишиЛинију!("   KRAJ IGRE");
        pišiLiniju!("━━━━━━━━━━━━━━━━━━━━━━");
    }
}
