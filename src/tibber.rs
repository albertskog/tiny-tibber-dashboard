use graphql_client::{GraphQLQuery, Response};
use std::env::var;
use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/query.graphql",
    response_derives = "Debug",
)]
struct PriceQuery;

pub fn get_prices() -> Vec<f64> {

    let client = reqwest::blocking::Client::new();
    let api_url = "https://api.tibber.com/v1-beta/gql";
    let api_token = var("TIBBER_API_TOKEN").unwrap();
    let request_body = PriceQuery::build_query(price_query::Variables);

    let res = client.post(api_url)
        .header("Authorization", api_token)
        .json(&request_body)
        .send().unwrap();
    let response_body: Response<price_query::ResponseData> = res.json().unwrap();
    // println!("{:#?}", response_body);

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let mut prices: Vec<f64> = Vec::new();

    for price in &response_body
        .data
        .as_ref()
        .expect("No data")
        .viewer
        .homes[0]
        .as_ref()
        .expect("No home")
        .current_subscription
        .as_ref()
        .expect("No subscription")
        .price_info
        .as_ref()
        .expect("No price info")
        .today
    {
        prices.push(*(price.as_ref().expect("no price").total.as_ref().expect("no total")));
    }
    println!("{:#?}", prices);

    prices
}