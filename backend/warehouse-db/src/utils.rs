//! Database utility functions

use warehouse_models::PaginationQuery;

/// Build dynamic sort clause for queries
pub fn build_sort_clause(
    sort_by: Option<&str>,
    sort_order: Option<&str>,
    default_sort: &str,
) -> String {
    let sort_column = match sort_by {
        Some("name") => "name",
        Some("code") => "code", 
        Some("created_at") => "created_at",
        Some("updated_at") => "updated_at",
        _ => default_sort,
    };
    
    let order = match sort_order {
        Some("DESC") | Some("desc") => "DESC",
        _ => "ASC",
    };
    
    format!("ORDER BY {} {}", sort_column, order)
}

/// Build search condition for text fields
pub fn build_search_condition(
    search: Option<&str>,
    fields: &[&str],
) -> (String, Vec<String>) {
    match search {
        Some(s) if !s.trim().is_empty() => {
            let search_term = format!("%{}%", s.trim());
            let conditions: Vec<String> = fields
                .iter()
                .enumerate()
                .map(|(i, field)| format!("{} ILIKE ${}", field, i + 1))
                .collect();
            
            let where_clause = format!("({})", conditions.join(" OR "));
            let params = vec![search_term; fields.len()];
            
            (where_clause, params)
        }
        _ => ("TRUE".to_string(), vec![]),
    }
}

/// Calculate pagination offset
pub fn calculate_offset(page: i64, limit: i64) -> i64 {
    (page.max(1) - 1) * limit.max(1)
}

/// Calculate total pages
pub fn calculate_total_pages(total: i64, limit: i64) -> i64 {
    if limit <= 0 {
        0
    } else {
        (total + limit - 1) / limit
    }
}

/// Validate pagination parameters
pub fn validate_pagination(query: &PaginationQuery) -> (i64, i64) {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).max(1).min(100); // Max 100 items per page
    (page, limit)
}
