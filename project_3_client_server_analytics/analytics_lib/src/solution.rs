use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

fn row_matches(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    match condition {
        Condition::Equal(col_name, expected_value) => {
            let idx = dataset.column_index(col_name);
            row.get_value(idx) == expected_value
        }
        Condition::Not(inner) => !row_matches(row, dataset, inner),
        Condition::And(left, right) => {
            row_matches(row, dataset, left) && row_matches(row, dataset, right)
        }
        Condition::Or(left, right) => {
            row_matches(row, dataset, left) || row_matches(row, dataset, right)
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut result = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if row_matches(row, dataset, filter) {
            result.add_row(row.clone());
        }
    }
    result
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let col_idx = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();
    let mut groups: HashMap<Value, Dataset> = HashMap::new();

    for row in dataset.into_iter() {
        let key = row.get_value(col_idx).clone();
        groups
            .entry(key)
            .or_insert_with(|| Dataset::new(columns.clone()))
            .add_row(row);
    }
    groups
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result: HashMap<Value, Value> = HashMap::new();

    for (group_value, grouped_dataset) in dataset {
        let aggregated_value = match aggregation {
            Aggregation::Count(_column_name) => {
                Value::Integer(grouped_dataset.len() as i32)
            }

            Aggregation::Sum(column_name) => {
                let col_idx = grouped_dataset.column_index(column_name);
                let mut sum = 0;

                for row in grouped_dataset.iter() {
                    match row.get_value(col_idx) {
                        Value::Integer(n) => {
                            sum += *n;
                        }
                        _ => panic!("Sum can only be used on integer columns"),
                    }
                }

                Value::Integer(sum)
            }

            Aggregation::Average(column_name) => {
                let col_idx = grouped_dataset.column_index(column_name);
                let mut sum = 0;

                for row in grouped_dataset.iter() {
                    match row.get_value(col_idx) {
                        Value::Integer(n) => {
                            sum += *n;
                        }
                        _ => panic!("Average can only be used on integer columns"),
                    }
                }

                let avg = sum / grouped_dataset.len() as i32;
                Value::Integer(avg)
            }
        };

        result.insert(group_value, aggregated_value);
    }

    result
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}