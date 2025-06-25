/// fr fr Advanced query builder with complex JOIN, subquery, and window function support
/// This module provides comprehensive SQL query building capabilities with Gen Z energy

use std::collections::HashMap;
use std::fmt::Display;
use tracing::{instrument, debug, info, trace};

use super::{DatabaseError, DatabaseErrorKind, SqlValue, QueryBuilder};

/// fr fr JOIN types supported by the advanced query builder
#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// fr fr Window function frame specification
#[derive(Debug, Clone)]
pub struct WindowFrame {
#[derive(Debug, Clone)]
pub enum FrameType {
#[derive(Debug, Clone)]
pub enum FrameBound {
/// fr fr Advanced SELECT query builder with complex features periodt
#[derive(Debug, Clone)]
pub struct AdvancedSelectBuilder {
    /// Selected columns and expressions
    /// FROM table or subquery
    /// JOIN clauses
    /// WHERE conditions
    /// GROUP BY fields
    /// HAVING conditions
    /// ORDER BY clauses
    /// LIMIT clause
    /// OFFSET clause
    /// Window functions
    /// CASE expressions
    /// Subqueries
    /// Union/Intersect operations
    /// Query parameters
#[derive(Debug, Clone)]
pub struct JoinClause {
#[derive(Debug, Clone)]
pub struct OrderByClause {
#[derive(Debug, Clone)]
pub enum OrderDirection {
#[derive(Debug, Clone)]
pub enum NullsOrder {
#[derive(Debug, Clone)]
pub struct WindowFunction {
#[derive(Debug, Clone)]
pub struct CaseExpression {
#[derive(Debug, Clone)]
pub struct WhenClause {
#[derive(Debug, Clone)]
pub struct SubqueryClause {
#[derive(Debug, Clone)]
pub struct SetOperation {
    pub all: bool, // For UNION ALL, INTERSECT ALL, etc.
#[derive(Debug, Clone)]
pub enum SetOperationType {
impl AdvancedSelectBuilder {
    /// slay Create new advanced SELECT builder
    #[instrument]
    pub fn new() -> Self {
        debug!("Creating new AdvancedSelectBuilder");
        Self {
        }
    }

    /// facts Select specific columns or expressions
    #[instrument(skip(self))]
    pub fn select(mut self, fields: &[&str]) -> Self {
        debug!(fields = ?fields, "Adding select fields");
        self.select_fields.extend(fields.iter().map(|f| f.to_string()));
        self
    /// highkey Select with aliases and complex expressions
    #[instrument(skip(self))]
    pub fn select_with_alias(mut self, expression: &str, alias: &str) -> Self {
        debug!(expression = expression, alias = alias, "Adding select field with alias");
        self.select_fields.push(format!("{} AS {}", expression, alias));
        self
    /// sus Set FROM table or subquery
    #[instrument(skip(self))]
    pub fn from(mut self, table: &str) -> Self {
        debug!(table = table, "Setting FROM clause");
        self.from_clause = Some(table.to_string());
        self
    /// lowkey Add INNER JOIN
    #[instrument(skip(self))]
    pub fn inner_join(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding INNER JOIN");
        self.joins.push(JoinClause {
        });
        self
    /// lowkey Add LEFT JOIN
    #[instrument(skip(self))]
    pub fn left_join(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding LEFT JOIN");
        self.joins.push(JoinClause {
        });
        self
    /// lowkey Add RIGHT JOIN
    #[instrument(skip(self))]
    pub fn right_join(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding RIGHT JOIN");
        self.joins.push(JoinClause {
        });
        self
    /// lowkey Add FULL OUTER JOIN
    #[instrument(skip(self))]
    pub fn full_outer_join(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding FULL OUTER JOIN");
        self.joins.push(JoinClause {
        });
        self
    /// yolo Add subquery in FROM clause
    #[instrument(skip(self, subquery))]
    pub fn from_subquery(mut self, subquery: AdvancedSelectBuilder, alias: &str) -> Self {
        debug!(alias = alias, "Adding subquery to FROM clause");
        self.subqueries.push(SubqueryClause {
        });
        self
    /// facts Add window function
    #[instrument(skip(self))]
    pub fn window_function(
    ) -> Self {
        debug!(
            "Adding window function"
        );
        
        let order_clauses = order_by
            .iter()
            .map(|(col, dir)| OrderByClause {
            })
            .collect();

        self.window_functions.push(WindowFunction {
        });
        self
    /// bestie Add CASE expression
    #[instrument(skip(self))]
    pub fn case_expression(mut self, alias: &str) -> CaseExpressionBuilder {
        debug!(alias = alias, "Starting CASE expression");
        CaseExpressionBuilder::new(self, alias.to_string())
    /// periodt Add UNION operation
    #[instrument(skip(self, other_query))]
    pub fn union(mut self, other_query: AdvancedSelectBuilder, all: bool) -> Self {
        debug!(all = all, "Adding UNION operation");
        self.set_operations.push(SetOperation {
        });
        self
    /// periodt Add INTERSECT operation
    #[instrument(skip(self, other_query))]
    pub fn intersect(mut self, other_query: AdvancedSelectBuilder, all: bool) -> Self {
        debug!(all = all, "Adding INTERSECT operation");
        self.set_operations.push(SetOperation {
        });
        self
    /// periodt Add EXCEPT operation
    #[instrument(skip(self, other_query))]
    pub fn except(mut self, other_query: AdvancedSelectBuilder, all: bool) -> Self {
        debug!(all = all, "Adding EXCEPT operation");
        self.set_operations.push(SetOperation {
        });
        self
    /// sus Add WHERE condition
    #[instrument(skip(self))]
    pub fn where_condition(mut self, condition: &str) -> Self {
        debug!(condition = condition, "Adding WHERE condition");
        self.where_conditions.push(condition.to_string());
        self
    /// facts Add parameterized WHERE condition
    #[instrument(skip(self))]
    pub fn where_param(mut self, condition: &str, value: SqlValue) -> Self {
        debug!(condition = condition, "Adding parameterized WHERE condition");
        self.where_conditions.push(condition.to_string());
        self.parameters.push(value);
        self
    /// lowkey Add GROUP BY field
    #[instrument(skip(self))]
    pub fn group_by(mut self, field: &str) -> Self {
        debug!(field = field, "Adding GROUP BY field");
        self.group_by.push(field.to_string());
        self
    /// highkey Add HAVING condition
    #[instrument(skip(self))]
    pub fn having(mut self, condition: &str) -> Self {
        debug!(condition = condition, "Adding HAVING condition");
        self.having_conditions.push(condition.to_string());
        self
    /// slay Add ORDER BY clause
    #[instrument(skip(self))]
    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        debug!(column = column, direction = ?direction, "Adding ORDER BY clause");
        self.order_by.push(OrderByClause {
        });
        self
    /// periodt Set LIMIT
    #[instrument(skip(self))]
    pub fn limit(mut self, limit: u64) -> Self {
        debug!(limit = limit, "Setting LIMIT");
        self.limit_clause = Some(limit);
        self
    /// periodt Set OFFSET
    #[instrument(skip(self))]
    pub fn offset(mut self, offset: u64) -> Self {
        debug!(offset = offset, "Setting OFFSET");
        self.offset_clause = Some(offset);
        self
    /// yolo Build the final query with optimizations
    #[instrument(skip(self))]
    pub fn build_optimized(&self) -> crate::error::Result<()> {
        info!("Building optimized advanced SQL query");
        
        let mut query_parts = Vec::new();
        
        // SELECT clause
        if self.select_fields.is_empty() {
            return Err(DatabaseError::query_error("No fields selected - what are we even querying bestie?"));
        query_parts.push(format!("SELECT {}", self.select_fields.join(", ")));
        
        // FROM clause
        if let Some(ref from) = self.from_clause {
            query_parts.push(format!("FROM {}", from));
        } else if !self.subqueries.is_empty() {
            // Use first subquery as FROM if no table specified
            let subquery = &self.subqueries[0];
            let (subquery_sql, _) = subquery.query.build_optimized()?;
            query_parts.push(format!("FROM ({}) AS {}", subquery_sql, subquery.alias));
        } else {
            return Err(DatabaseError::query_error("No FROM clause specified - gotta select from somewhere periodt"));
        // JOIN clauses
        for join in &self.joins {
            query_parts.push(format!("{} {} ON {}", join.join_type, join.table, join.on_condition));
        // WHERE clause
        if !self.where_conditions.is_empty() {
            query_parts.push(format!("WHERE {}", self.where_conditions.join(" AND ")));
        // GROUP BY clause
        if !self.group_by.is_empty() {
            query_parts.push(format!("GROUP BY {}", self.group_by.join(", ")));
        // HAVING clause
        if !self.having_conditions.is_empty() {
            query_parts.push(format!("HAVING {}", self.having_conditions.join(" AND ")));
        // ORDER BY clause
        if !self.order_by.is_empty() {
            let order_items: Vec<String> = self.order_by
                .iter()
                .map(|order| {
                    let direction = match order.direction {
                    format!("{} {}", order.column, direction)
                })
                .collect();
            query_parts.push(format!("ORDER BY {}", order_items.join(", ")));
        // LIMIT clause
        if let Some(limit) = self.limit_clause {
            query_parts.push(format!("LIMIT {}", limit));
        // OFFSET clause
        if let Some(offset) = self.offset_clause {
            query_parts.push(format!("OFFSET {}", offset));
        // Set operations (UNION, INTERSECT, EXCEPT)
        let mut final_query = query_parts.join(" ");
        for set_op in &self.set_operations {
            let (other_query, _) = set_op.query.build_optimized()?;
            let operation = match set_op.operation_type {
            final_query = format!("{} {} {}", final_query, operation, other_query);
        trace!(query = %final_query, params = ?self.parameters, "Built advanced SQL query");
        Ok((final_query, self.parameters.clone()))
    }
}

impl QueryBuilder for AdvancedSelectBuilder {
    #[instrument(skip(self))]
    fn build(&self) -> crate::error::Result<()> {
        let (query, _) = self.build_optimized()?;
        Ok(query)
    fn get_parameters(&self) -> Vec<SqlValue> {
        self.parameters.clone()
    fn validate(&self) -> crate::error::Result<()> {
        if self.select_fields.is_empty() {
            return Err(DatabaseError::query_error("SELECT clause cannot be empty"));
        if self.from_clause.is_none() && self.subqueries.is_empty() {
            return Err(DatabaseError::query_error("FROM clause is required"));
        Ok(())
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr CASE expression builder for fluent SQL building
pub struct CaseExpressionBuilder {
impl CaseExpressionBuilder {
    fn new(parent: AdvancedSelectBuilder, alias: String) -> Self {
        Self {
        }
    }

    /// yolo Add WHEN condition
    #[instrument(skip(self))]
    pub fn when(mut self, condition: &str, result: &str) -> Self {
        debug!(condition = condition, result = result, "Adding WHEN clause");
        self.when_clauses.push(WhenClause {
        });
        self
    /// periodt Add ELSE clause
    #[instrument(skip(self))]
    pub fn else_clause(mut self, result: &str) -> Self {
        debug!(result = result, "Adding ELSE clause");
        self.else_clause = Some(result.to_string());
        self
    /// facts Finish building CASE expression
    #[instrument(skip(self))]
    pub fn end_case(mut self) -> AdvancedSelectBuilder {
        debug!(alias = %self.alias, "Completing CASE expression");
        
        let case_expr = CaseExpression {
        
        self.parent.case_expressions.push(case_expr);
        
        // Add to SELECT fields
        let case_sql = self.build_case_sql();
        self.parent.select_fields.push(format!("{} AS {}", case_sql, self.alias));
        
        self.parent
    fn build_case_sql(&self) -> String {
        let mut case_parts = Vec::from(["CASE".to_string()]);
        
        for when_clause in &self.when_clauses {
            case_parts.push(format!("WHEN {} THEN {}", when_clause.condition, when_clause.result));
        if let Some(ref else_result) = self.else_clause {
            case_parts.push(format!("ELSE {}", else_result));
        case_parts.push("END".to_string());
        case_parts.join(" ")
    }
}

impl Default for AdvancedSelectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

