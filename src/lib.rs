
extern crate horned_owl;
extern crate petgraph;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;
use horned_owl::model::*;
use horned_owl::visitor::Visit;
use horned_owl::model::ForIRI;
use petgraph::graph::{Graph, NodeIndex};
use std::hash::Hash;

pub trait RenderOnt<A: ForIRI> {
    fn render_ontology(&mut self) {}
}

pub struct TaxonomyGraph<A,I> where A: Default + Eq + PartialEq + Hash + Copy, I: ForIRI {
    nodes: Vec<I>,
    edges: Vec<(I,I)>,
    map: HashMap<I,Option<A>>
    
}

impl<A,I> Default for TaxonomyGraph<A,I> where
     A: Default + Eq + PartialEq + Hash + Copy,
     I: ForIRI {
    fn default() -> Self {
        TaxonomyGraph { nodes: vec![], edges: vec![], map: HashMap::new()}
}
}

impl<A,I> TaxonomyGraph<A,I> where 
A: Default + Eq + PartialEq + Hash + Copy, I: ForIRI {
    pub fn into_graph(&self) -> Graph<A,()> {
        let mut graph = Graph::<A, ()>::new();
        let mut node_ix = HashMap::<A,NodeIndex>::new();
        for n in &self.nodes {
            let name = self.map.get(n).unwrap().unwrap();
            node_ix.insert(name, graph.add_node(name));
        }
        for (a, b) in &self.edges {
            let left = node_ix.get(a).unwrap();
            let right = node_ix.get(b).unwrap();
            graph.update_edge(*left, *right, ());
        }
        graph
    }
}

impl<I: ForIRI, A: Default + Eq + PartialEq + Hash + Copy> Visit<I> for TaxonomyGraph<A,I>{
    fn visit_class(&mut self, class: &Class<I>) {
        self.nodes.push(class.into());
        let key = class.try_into().unwrap();
        match self.map.entry(key) {
            Entry::Occupied(o) => (),
            Entry::Vacant(v) => {v.insert(None);}
        };
    }

    fn visit_sub_class_of(&mut self, ex: &SubClassOf<I>) {
        
        let sup = &ex.sup;
        let sup_class = match sup {
            ClassExpression::Class(e) => Some(e),
            _ => None
        };
        let sub = &ex.sub;
        let sub_class = match sub {
            ClassExpression::Class(e) => Some(e),
            _ => None
        };
        if sup_class.is_some() & sub_class.is_some() {
            
        }
        
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
        let mut  tax: TaxonomyGraph<&str,String> = TaxonomyGraph::default();
        let a_node ="a".into(); 
        let b_node = "b".into();
        tax.nodes.push(a_node);
        tax.nodes.push(b_node);
        tax.edges.push((a_node, b_node));
        tax.map.insert(a_node, Some("a"));
        tax.map.insert(b_node, Some("b"));
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
