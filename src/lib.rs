extern crate horned_owl;
extern crate petgraph;

use horned_owl::model::ForIRI;
use horned_owl::model::*;
use horned_owl::visitor::Visit;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::{hash_map::Entry, HashMap};

const RDFSLABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";

pub trait RenderOnt<A: ForIRI> {
    fn render_ontology(&mut self) {}
}

#[derive(Debug)]
pub struct TaxonomyGraph<I>
where
    I: ForIRI,
{
    nodes: Vec<I>,
    edges: Vec<(I, I)>,
    map: HashMap<I, String>,
}

impl<I> Default for TaxonomyGraph<I>
where
    I: ForIRI,
{
    fn default() -> Self {
        TaxonomyGraph {
            nodes: vec![],
            edges: vec![],
            map: HashMap::<I, String>::new(),
        }
    }
}

impl<I> TaxonomyGraph<I>
where
    I: ForIRI,
{
    pub fn into_graph(&self) -> Graph<String, ()> {
        let mut graph = Graph::<String, ()>::new();
        let mut node_ix = HashMap::<String, NodeIndex>::new();
        for n in &self.nodes {
            let name: String = match self.map.get(n.as_ref()) {
                Some(nn) => nn.clone(),
                None => n.as_ref().into(),
            };
            match node_ix.entry(name.clone()) {
                Entry::Occupied(_) => (),
                Entry::Vacant(v) => {
                    v.insert(graph.add_node(name.clone()));
                }
            };
        }
        for (a, b) in &self.edges {
            let a_name: String = match self.map.get(a.as_ref()) {
                Some(nn) => nn.clone(),
                None => a.as_ref().into(),
            };
            let b_name: String = match self.map.get(b.as_ref()) {
                Some(nn) => nn.clone(),
                None => b.as_ref().into(),
            };
            let left = node_ix.get(&a_name).unwrap();
            let right = node_ix.get(&b_name).unwrap();
            graph.update_edge(*left, *right, ());
        }
        graph
    }
}

impl<I: ForIRI> Visit<I> for TaxonomyGraph<I> {
    fn visit_class(&mut self, class: &Class<I>) {
        self.nodes.push(class.0.underlying());
    }

    fn visit_sub_class_of(&mut self, ex: &SubClassOf<I>) {
        let sup = &ex.sup;
        let sup_class = match sup {
            ClassExpression::Class(e) => Some(e),
            _ => None,
        };
        let sub = &ex.sub;
        let sub_class = match sub {
            ClassExpression::Class(e) => Some(e),
            _ => None,
        };
        if sup_class.is_some() & sub_class.is_some() {
            self.edges.push((
                sup_class.unwrap().0.underlying(),
                sub_class.unwrap().0.underlying(),
            ))
        }
    }

    fn visit_annotation_assertion(&mut self, aa: &AnnotationAssertion<I>) {
        if aa.ann.ap.0.as_ref() == RDFSLABEL {
            match &aa.subject {
                AnnotationSubject::IRI(iri) => {
                    let literal = match &aa.ann.av {
                        AnnotationValue::Literal(l) => Some(l.literal()),
                        _ => None,
                    };
                    if literal.is_some() {
                        match self.map.entry(iri.underlying()) {
                            Entry::Occupied(_) => (),
                            Entry::Vacant(v) => {
                                v.insert(literal.unwrap().clone().into());
                            }
                        };
                    }
                }
                _ => (),
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use horned_owl::io::owx::reader::read_with_build;
    use horned_owl::model::Build;
    use horned_owl::ontology::set::SetOntology;
    use horned_owl::visitor::Walk;
    use petgraph::dot::{Config, Dot};

    use std::io::BufRead;

    pub fn read_ok<R: BufRead>(bufread: &mut R) -> SetOntology<String> {
        let r = read_with_build(bufread, &Build::new_string());
        assert!(r.is_ok(), "Expected ontology, got failure:{:?}", r.err());
        let (o, _) = r.ok().unwrap();

        o
    }

    #[test]
    fn test_graph_building() {
        let mut tax: TaxonomyGraph<String> = TaxonomyGraph::default();
        let a_node: String = "a".into();
        let b_node: String = "b".into();
        tax.nodes.push(a_node.clone());
        tax.nodes.push(b_node.clone());
        tax.edges.push((a_node, b_node));
        let a_node: String = "a".into();
        let b_node: String = "b".into();
        tax.map.insert(a_node, "a".into());
        tax.map.insert(b_node, "b".into());
        let g = tax.into_graph();
        println!("{:?}", g);
    }

    #[test]
    fn build_graph() {
        let ont_s = include_str!("../tmp/bfo.owx");
        let ont = read_ok(&mut ont_s.as_bytes());

        let mut walk: Walk<String, TaxonomyGraph<String>> = Walk::new(TaxonomyGraph::default());
        walk.set_ontology(&ont);
        println!(
            "{:?}",
            Dot::with_config(&walk.into_visit().into_graph(), &[Config::EdgeNoLabel])
        );
        assert!(true);
    }
}
