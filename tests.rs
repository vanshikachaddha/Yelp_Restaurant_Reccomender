use crate::make_graphs::{good_make_graph, bad_make_graph, top_reviewers, top_restaurants, create_adjacency_matrix, restaurant_recommender};
use std::collections::{HashMap, HashSet};
use crate::make_graphs::Graph;

use super::*;

#[test]
fn test_good_make_graph() {
    let reviews = vec![
        ("user1".to_string(), "business1".to_string(), 4.0),
        ("user2".to_string(), "business2".to_string(), 3.0),
        ("user1".to_string(), "business3".to_string(), 5.0),
    ];
    let graph = good_make_graph(reviews);

    assert_eq!(graph.users.len(), 1);
    assert!(graph.users.contains("user1"));
    assert_eq!(graph.businesses.len(), 2);
    assert!(graph.businesses.contains("business1"));
}

#[test]
fn test_bad_make_graph() {
    let reviews = vec![
        ("user1".to_string(), "business1".to_string(), 2.5),
        ("user2".to_string(), "business2".to_string(), 3.0),
        ("user3".to_string(), "business3".to_string(), 4.0),
    ];
    let graph = bad_make_graph(reviews);

    assert_eq!(graph.users.len(), 2);
    assert!(graph.users.contains("user1"));
    assert_eq!(graph.businesses.len(), 2);
    assert!(graph.businesses.contains("business2"));
}

#[test]
fn test_top_reviewers() {
    let mut edges = HashMap::new();
    edges.insert("user1".to_string(), vec!["business1".to_string(), "business2".to_string()]);
    edges.insert("user2".to_string(), vec!["business3".to_string()]);
    
    let reviewers = top_reviewers(&edges);
    assert_eq!(reviewers["user1"], 2);
    assert_eq!(reviewers["user2"], 1);
}

#[test]
fn test_top_restaurants() {
    let graph = Graph {
        users: HashSet::new(),
        businesses: HashSet::new(),
        edges: HashMap::from([
            ("user1".to_string(), vec!["business1".to_string(), "business2".to_string()]),
            ("user2".to_string(), vec!["business1".to_string()]),
        ]),
    };

    let restaurants = top_restaurants(&graph);
    assert_eq!(restaurants["business1"], 2);
    assert_eq!(restaurants["business2"], 1);
}

#[test]
fn test_create_adjacency_matrix() {
    let graph = HashMap::from([
        ("user1".to_string(), vec!["business1".to_string(), "business2".to_string()]),
        ("user2".to_string(), vec!["business2".to_string()]),
    ]);

    let matrix = create_adjacency_matrix(&graph);
    assert!(matrix.contains_key("user1"));
    assert_eq!(matrix["user1"], vec![1, 1]);
}

#[test]
fn test_restaurant_recommender() {
    let adjacency_matrix = HashMap::from([
        ("user1".to_string(), vec![1, 0, 1]),
        ("user2".to_string(), vec![0, 1, 0]),
        ("user3".to_string(), vec![1, 1, 0]),
    ]);
    let similarity_scores = HashMap::from([
        (("user1".to_string(), "user3".to_string()), 0.9),
    ]);
    let business_index_to_name = HashMap::from([
        (0, "business1".to_string()),
        (1, "business2".to_string()),
        (2, "business3".to_string()),
    ]);

    let recommendations = restaurant_recommender(
        &"user1".to_string(),
        &similarity_scores,
        &adjacency_matrix,
        &business_index_to_name,
    );
    assert_eq!(recommendations, vec!["business2".to_string()]);
}
