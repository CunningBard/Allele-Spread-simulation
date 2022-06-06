use crate::essential_functions::{rand_prob};

mod essential_functions;

#[derive(Clone)]
struct Allele
{
    is_dominant: bool,
    name: char
}

#[derive(Clone)]
struct AlleleDuo
{
    left_allele: Allele,
    right_allele: Allele,
}

impl AlleleDuo {
    fn is_co_dominant(&self) -> bool {
        self.left_allele.is_dominant && self.right_allele.is_dominant
    }
    fn pheno_type(&self) -> String { format!("{}{}", self.left_allele.name, self.right_allele.name) }
}

struct Person
{
    alleles: Vec<AlleleDuo>
}

impl Person {
    fn get_alleles_for_replication(&self) -> Vec<Allele>{
        let mut send_alleles = vec![];
        for duos in &self.alleles {
            if rand_prob(50){
                send_alleles.push(duos.left_allele.clone())
            } else {
                send_alleles.push(duos.right_allele.clone())
            }
        }
        send_alleles
    }
}

fn main() {

    println!("Hello, world!");
}
