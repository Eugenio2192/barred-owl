
extern crate horned_owl;
extern crate petgraph;

use std::collections::HashMap;
use horned_owl::model::*;
use horned_owl::visitor::Visit;
use horned_owl::model::ForIRI;
use horned_owl::model::IRI;
use petgraph::adj::*;
use petgraph::graph::Graph;
use petgraph::adj::IndexType;
use std::io::Write;

pub trait RenderOnt<A: ForIRI> {
    fn render_ontology(&mut self) {}
}

pub struct TaxonomyGraph<A, K>{
    nodes: Vec<A>,
    edges: Vec<(A,A)>,
    map: HashMap<K,A>
    
}
// pub struct RLOntologyFilter(ComponentMappedOntology);
impl<A,K> Default for TaxonomyGraph<A,K> {
    fn default() -> Self {
        TaxonomyGraph { nodes: vec![], edges: vec![], map: HashMap::new() }
}
}

impl<A,K> TaxonomyGraph<A,K> where 
    A: Into<NodeIndex> + Default + Copy{
    pub fn into_graph(self) -> Graph<A, A> {
        Graph::<A, A>::from_edges(&self.edges)
    }
}
#[cfg(test)]
mod test {
    use horned_owl::io::owx::reader::read_with_build;
    use horned_owl::model::Build;
    use horned_owl::ontology::set::SetOntology;
    use horned_owl::visitor::Walk;
    use super::*;

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

        let mut walk = Walk::new(TaxonomyGraph::default());
        walk.set_ontology(&ont);
        let mut v = walk.into_visit().into_vec();
        v.sort();
        for val in v {
            println!("{:?}", val);
        }
        assert!(true);
    }
}
