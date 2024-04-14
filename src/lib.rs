
extern crate horned_owl;
extern crate petgraph;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;
use horned_owl::model::*;
use horned_owl::visitor::Visit;
use horned_owl::model::ForIRI;
use petgraph::graph::{Graph, NodeIndex};
use std::io::Write;
use std::hash::Hash;

pub trait RenderOnt<A: ForIRI> {
    fn render_ontology(&mut self) {}
}

pub struct TaxonomyGraph<A> where A: Default + Eq + PartialEq + Hash + Copy {
    nodes: Vec<A>,
    edges: Vec<(A,A)>,
    map: HashMap<String,Option<A>>
    
}
// pub struct RLOntologyFilter(ComponentMappedOntology);
impl<A> Default for TaxonomyGraph<A> where A: Default + Eq + PartialEq + Hash + Copy {
    fn default() -> Self {
        TaxonomyGraph { nodes: vec![], edges: vec![], map: HashMap::new()}
}
}

impl<A> TaxonomyGraph<A> where 
A: Default + Eq + PartialEq + Hash + Copy {
    pub fn into_graph(&self) -> Graph<A,()> {
        let mut graph = Graph::<A, ()>::new();
        let mut node_ix = HashMap::<A,NodeIndex>::new();
        for n in &self.nodes {
            node_ix.insert(*n, graph.add_node(*n));
        }
        for (a, b) in &self.edges {
            let left = node_ix.get(a).unwrap();
            let right = node_ix.get(b).unwrap();
            graph.update_edge(*left, *right, ());
        }
        graph
    }
}

impl<I: ForIRI, A: Default + Eq + PartialEq + Hash + Copy> Visit<I> for TaxonomyGraph<A>{
    fn visit_class(&mut self, class: &Class<I>) {
        let key = class.try_into().unwrap();
        match self.map.entry(key) {
            Entry::Occupied(o) => (),
            Entry::Vacant(v) => {v.insert(None);}
        };
    }

    fn visit_annotated_component(&mut self, _: &AnnotatedComponent<I>) {
        
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
    fn test_graph_building() {
        let mut  tax: TaxonomyGraph<&str> = TaxonomyGraph::default();
        tax.nodes.push("a");
        tax.nodes.push("b");
        tax.edges.push(("a", "b"));
        tax.map.insert("iri:a".into(), Some("a"));
        tax.map.insert("iri:b".into(), Some("b"));
        let g = tax.into_graph();
        println!("{:?}", g);

    }

    #[test]
    fn single_class() {
        let ont_s = include_str!("../tmp/bfo.owx");
        let ont = read_ok(&mut ont_s.as_bytes());

        let mut walk: Walk<String, TaxonomyGraph<&str>> = Walk::new(TaxonomyGraph::default());
        walk.set_ontology(&ont);
        // let mut v = walk.into_visit().into_vec();
        // v.sort();
        // for val in v {
        //     println!("{:?}", val);
        // }
        // assert!(true);
    }
}
