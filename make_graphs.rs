use std::collections::{HashMap, HashSet};
use csv::Reader;
use std::error::Error;
use itertools::Itertools;
#[derive(Debug)]

// Nodes: Users and Businesses
pub struct Graph {
    pub users: HashSet<String>,
    pub businesses: HashSet<String>,
    // {user_id, [business_ids]}
    pub edges: HashMap<String, Vec<String>> 
}

impl Graph {
    // Method to iterate over edges
    pub fn edges_iter(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.edges.iter()
    }
}

// Reading Reviews CSV File

pub fn read_reviews(filepath: &str) -> Result<Vec<(String, String, f64)>, Box<dyn Error>> {
    let mut review = Vec::new();
    let mut result = Reader::from_path(filepath)?;

    for data in result.records() {
        let record = data?;
        let user_id = record.get(1).unwrap_or_default().to_string();
        let business_id = record.get(2).unwrap_or_default().to_string();
        let stars: f64 = record.get(3).unwrap_or_default().parse()?;
        review.push((user_id, business_id, stars));
        
    }

    Ok(review)
}


// Creating a Graph Based on Extracted Reviews
pub fn good_make_graph(review: Vec<(String, String, f64)>) -> Graph {
    let mut users = HashSet::new();
    let mut businesses = HashSet::new();
    let mut edges = HashMap::new();

    for (user_id, business_id, stars) in review {
        if (stars>=3.5) {
            users.insert(user_id.clone());
            businesses.insert(business_id.clone());
            edges.entry(user_id.clone()).or_insert(Vec::new()).push(business_id);
        }
    }

    Graph {users, businesses, edges}
}

pub fn bad_make_graph(review: Vec<(String, String, f64)>) -> Graph {
    let mut users = HashSet::new();
    let mut businesses = HashSet::new();
    let mut edges = HashMap::new();

    for (user_id, business_id, stars) in review {
        if (stars<3.5) {
            users.insert(user_id.clone());
            businesses.insert(business_id.clone());
            edges.entry(user_id.clone()).or_insert(Vec::new()).push(business_id);
        }
    }

    Graph{users, businesses, edges}
}

//Figuring out Degrees of Distribution
pub fn top_reviewers(graph: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    for (node, businesses) in graph {
        count.insert(node.clone(), businesses.len());
    }
    //count = (user_id, restaurants_reviewed)
    count
}

pub fn top_restaurants(graph: &Graph) -> HashMap<String, usize> {
    let mut count: HashMap<String, usize> = HashMap::new();
    for businesses in graph.edges.values() {
        for business_id in businesses {
            * count.entry(business_id.clone()).or_insert(0)+=1;
        }
    }

    //count = (business_id, number_of_reviews)
    count     
}

//Finding Top 'n'
pub fn top_nodes(data: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut arr: Vec<_> = data.iter().collect();
    arr.sort_by(|a, b| b.1.cmp(a.1));
    arr.into_iter()
        .take(n)
        .map(|(key, value)| (key.clone(), *value))
        .collect()
}

//Friends of Friends

//Creating User Business Adjacency Matrix
pub fn create_adjacency_matrix(graph: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<usize>> {
    let mut all_businesses: HashSet<String> = HashSet::new();
    let mut adjacency_matrix: HashMap<String, Vec<usize>> = HashMap::new();
    for business_list in graph.values() {
        for business_id in business_list {
            all_businesses.insert(business_id.clone());
        }
    }

    let business_list: Vec<_> = all_businesses.iter().collect();
    let business_index: HashMap<String, usize> = business_list.iter().enumerate().map(|(i, business)| (business.to_string(), i)).collect();

    for (user, business) in graph {
        let mut row = vec![0;business_list.len()];
        for business_id in business {
            let index = business_index.get(business_id).unwrap();
            row[*index] = 1;
        }
        adjacency_matrix.insert(user.clone(), row);
    }

    adjacency_matrix
}

// Cosine Similarity between 2 Users
pub fn precompute_norms(adjacency_matrix: &HashMap<String, Vec<usize>>) -> HashMap<String, f64> {
    adjacency_matrix
        .iter()
        .map(|(user_id, vec)| {
            let norm = vec.iter().map(|&x| (x * x) as f64).sum::<f64>().sqrt();
            (user_id.clone(), norm)
        })
        .collect()
}

pub fn cosine_similarity(
    user_id1: &String,
    user_id2: &String,
    adjacency_matrix: &HashMap<String, Vec<usize>>,
    precomputed_norms: &HashMap<String, f64>,
) -> f64 {
    let vec1 = adjacency_matrix.get(user_id1).unwrap();
    let vec2 = adjacency_matrix.get(user_id2).unwrap();

    let dot_product: usize = vec1.iter().zip(vec2.iter()).map(|(x, y)| x * y).sum();
    let norm1 = *precomputed_norms.get(user_id1).unwrap();
    let norm2 = *precomputed_norms.get(user_id2).unwrap();

    if norm1 == 0.0 || norm2 == 0.0 {
        return 0.0;
    }

    dot_product as f64 / (norm1 * norm2)
}

// Getting Most Similar Users
pub fn calculate_user_similarity(
    adjacency_matrix: &HashMap<String, Vec<usize>>,
    precomputed_norms: &HashMap<String, f64>,
) -> HashMap<(String, String), f64> {
    let mut scores: HashMap<(String, String), f64> = HashMap::new();

    // Step 1: Create a business-to-user mapping
    let mut business_to_users: HashMap<usize, Vec<&String>> = HashMap::new();
    for (user_id, user_vector) in adjacency_matrix {
        for (business_index, &value) in user_vector.iter().enumerate() {
            if value == 1 {
                business_to_users
                    .entry(business_index)
                    .or_insert_with(Vec::new)
                    .push(user_id);
            }
        }
    }

    // Step 2: Only compute similarities for users who share businesses
    for users in business_to_users.values() {
        for i in 0..users.len() {
            for j in (i + 1)..users.len() {
                let user1 = users[i];
                let user2 = users[j];

                // Compute cosine similarity
                let similarity_score = cosine_similarity(user1, user2, adjacency_matrix, precomputed_norms);
                if similarity_score > 0.0 {
                    scores.insert((user1.clone(), user2.clone()), similarity_score);
                }
            }
        }
    }

    scores
}

pub fn restaurant_recommender(
    user_id: &String,
    similarity_scores: &HashMap<(String, String), f64>,
    adjacency_matrix: &HashMap<String, Vec<usize>>,
    business_index_to_name: &HashMap<usize, String>,
) -> Vec<String> {
    // Step 1: Get businesses reviewed by the target user
    let reviewed_businesses: HashSet<usize> = adjacency_matrix
        .get(user_id)
        .unwrap_or_else(|| panic!("User ID {} not found in adjacency matrix", user_id))
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| if value == 1 { Some(index) } else { None })
        .collect();

    // Step 2: Aggregate scores for businesses reviewed by similar users
    let mut candidate_scores: HashMap<usize, f64> = HashMap::new();
    for (&(ref user_a, ref user_b), &similarity) in similarity_scores {
        // Skip irrelevant pairs
        let similar_user = if *user_a == *user_id {
            user_b
        } else if *user_b == *user_id {
            user_a
        } else {
            continue; // Skip if neither user matches
        };

        // Retrieve the similar user's vector
        if let Some(similar_user_vector) = adjacency_matrix.get(similar_user) {
            for (business_index, &value) in similar_user_vector.iter().enumerate() {
                if value == 1 && !reviewed_businesses.contains(&business_index) {
                    // Increment the candidate score for this business
                    *candidate_scores.entry(business_index).or_insert(0.0) += similarity;
                }
            }
        }
    }

    // Step 3: Convert to vector and sort
    let mut ranked_businesses: Vec<(usize, f64)> = candidate_scores.into_iter().collect();
    ranked_businesses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Step 4: Map to business names and return top 10
    ranked_businesses
        .into_iter()
        .take(10) // Limit to top 10 results
        .filter_map(|(business_index, _)| business_index_to_name.get(&business_index).cloned())
        .collect()
}

pub fn read_business_names(filepath: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut business_map = HashMap::new();
    let mut reader = Reader::from_path(filepath)?;

    for result in reader.records() {
        let record = result?;
        let business_id = record.get(0).unwrap_or_default().to_string();
        let name = record.get(1).unwrap_or_default().to_string();
        business_map.insert(business_id, name);
    }

    Ok(business_map)
}
