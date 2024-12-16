/* Vanshika Chaddha
    CDS DS210: Final Project
    Yelp Restaurants Reccomendation System
    DUE: DEC 15, 2022
*/

#[cfg(test)]
mod tests;
pub mod make_graphs;
use make_graphs::read_reviews;
use std::collections::{HashMap, HashSet};
use make_graphs::{read_business_names, good_make_graph, bad_make_graph, top_restaurants, top_reviewers, top_nodes, create_adjacency_matrix, cosine_similarity, calculate_user_similarity, restaurant_recommender, precompute_norms};

fn main() {
    let reviews_path = "CSV_Dataset/Archive/filtered_reviews.csv";
    let business_names_path = "CSV_Dataset/Archive/filtered_restaurants.csv";
    let reviews = read_reviews(reviews_path).unwrap();
    let business_names = read_business_names(business_names_path).unwrap();

    //Creating Graphs for Good and Bad Ratings
    let good_ratings = good_make_graph(reviews.clone());
    let bad_ratings = bad_make_graph(reviews.clone());

    //Finding Top Reviewers and Restaurants (Good)
    let top_good_reviewers = top_reviewers(&good_ratings.edges);
    let top_good_restaurants = top_restaurants(&good_ratings);

    //Finding Top Reviewers and Restaurants (Bad)
    let top_bad_reviewers = top_reviewers(&bad_ratings.edges);
    let top_bad_restaurants = top_restaurants(&bad_ratings);

    //Printing Results
    println!("Top 20 Reviewers for Good Ratings:");
    for (user, count) in top_nodes(&top_good_reviewers, 20) {
        println!("User: {}, Reviews: {}", user, count);
    }

    println!("Top 20 Restaurants for Good Ratings:");
        for (user, count) in top_nodes(&top_good_restaurants, 20) {
            println!("Restaurant: {}, Reviews: {}", user, count);
        }

    println!("Top 20 Restaurants for Bad Ratings:");
    for (user, count) in top_nodes(&top_bad_restaurants, 20) {
        println!("Restaurant: {}, Reviews: {}", user, count);
    }

    //Finding Restaurant Reccomendations
    let adjacency_matrix_good = create_adjacency_matrix(&good_ratings.edges);

    let all_businesses: HashSet<_> = good_ratings.businesses.iter().cloned().collect();
    let business_list: Vec<_> = all_businesses.iter().collect();
    let business_index_to_name: HashMap<usize, String> = business_list
        .iter()
        .enumerate()
        .map(|(i, business)| (i, (*business).to_string()))
        .collect();

    let precomputed_norms = precompute_norms(&adjacency_matrix_good);

    let user_similarities_good = calculate_user_similarity(&adjacency_matrix_good, &precomputed_norms);

    let target_user_id = "gvXtMj3XuPr0xHjgmlmtng".to_string(); // Replace with your target user ID
    let recommendations = restaurant_recommender(
        &target_user_id,
        &user_similarities_good,
        &adjacency_matrix_good,
        &business_index_to_name,
    );

    println!("\nRecommended Restaurants for {}:", target_user_id);
    for business_id in recommendations {
        if let Some(name) = business_names.get(&business_id) {
            println!("{} - {}", business_id, name);
        } else {
            println!("{} - [Name not found]", business_id);
        }
    }
}
