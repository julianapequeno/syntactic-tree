#[derive(Debug)]
enum Operador {
    Adicao,
    Subtracao,
}

#[derive(Debug)]
enum Expressao {
    Numero(i64),
    OperadorBinario {
        operador: Operador,
        esquerda: Box<Expressao>,
        direita: Box<Expressao>,
    },
    OperadorUnario(Box<Expressao>),
}

impl Expressao {
    fn avaliar(&self) -> Option<i64> {
        match self {
            Expressao::Numero(i) => Some(*i),
            Expressao::OperadorUnario(e) => e.avaliar()?.checked_neg(),
            Expressao::OperadorBinario { operador, esquerda, direita} => {
                let e = esquerda.avaliar()?;
                let d = direita.avaliar()?;
                let op = match operador {
                    Operador::Adicao => i64::checked_add,
                    Operador::Subtracao => i64::checked_sub,
                };
                op(e,d)
            }
        }
    }

    fn imprimir_nodo(&self, profundidade: usize){
        for _ in 0..profundidade {
            print!("|");
        }

        match self {
            Expressao::Numero(i) => println!("{}", *i),
            Expressao::OperadorBinario { operador, esquerda, direita} => {
                match operador {
                    Operador::Adicao => println!("+"),
                    Operador::Subtracao => println!("-"),
                }
                esquerda.imprimir_nodo(profundidade + 1);
                direita.imprimir_nodo( profundidade + 1);
            }
            Expressao::OperadorUnario(u) => {
                println!("-");
                u.imprimir_nodo(profundidade + 1);
            }
        }
    }

    fn imprimir(&self) {
    }

    fn imprimir_arvore(&self) {
        self.imprimir_nodo(0);
    }
}


fn main() {
   let n10 = Expressao::Numero(10); // i64::MAX
    let n20 = Expressao::Numero(20); //i64::MIN + 1
    let e = Expressao::OperadorBinario {
        operador: Operador::Subtracao,
        esquerda: Box::new(Expressao::Numero(50)),
        direita: Box::new(n20),
    };

    let e = Expressao::OperadorBinario {
        operador: Operador::Adicao,
        esquerda: Box::new(n10),
        direita: Box::new(e),
    };

    let e = Expressao::OperadorUnario(Box::new(e));
    e.imprimir_arvore();
    println!("{e:?}");
    println!("Resultado: {:?}",e.avaliar());
}
