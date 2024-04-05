
extern crate horned_owl;


pub mod entity {
    use horned_owl::visitor::Visit;
    use horned_owl::model::ForIRI;
    use horned_owl::model::IRI;
    use horned_owl::model::AnonymousIndividual;
    use horned_owl::ontology::component_mapped::ComponentMappedOntology;

    // pub struct RLOntologyFilter(ComponentMappedOntology);
}

#[cfg(test)]
mod test {
    use horned_owl::io::owx::reader::read_with_build;
    use horned_owl::model::Build;
    use horned_owl::ontology::set::SetOntology;
    use horned_owl::visitor::*;

    use std::io::BufRead;

    pub fn read_ok<R: BufRead>(bufread: &mut R) -> SetOntology<String> {
        let r = read_with_build(bufread, &Build::new_string());
        assert!(r.is_ok(), "Expected ontology, got failure:{:?}", r.err());
        let (o, _) = r.ok().unwrap();

        o
    }

    #[test]
    fn single_class() {
        let ont_s = include_str!("../tmp/bfo.owx");
        let ont = read_ok(&mut ont_s.as_bytes());

        let mut walk = Walk::new(entity::IRIExtract::default());
        walk.set_ontology(&ont);
        let mut v = walk.into_visit().into_vec();
        v.sort();
        for val in v {
            println!("{:?}", val);
        }
        assert!(true);
    }
}
