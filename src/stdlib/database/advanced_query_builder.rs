/// fr fr Advanced query builder with complex JOIN, subquery, and window function support
/// This module provides comprehensive SQL query building capabilities with Gen Z energy

use std::collections::HashMap;
use std::fmt::Display;
use tracing::{instrument, debug, info, trace};

use super::{DatabaseError, DatabaseErrorKind, SqlValue, QueryBuilder};

/// fr fr JOIN types supported by the advanced query builder
#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    FullOuter,
    Cross,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinType::Inner => write!(f, "INNER JOIN"),
            JoinType::Left => write!(f, "LEFT JOIN"),
            JoinType::Right => write!(f, "RIGHT JOIN"),
            JoinType::FullOuter => write!(f, "FULL OUTER JOIN"),
            JoinType::Cross => write!(f, "CROSS JOIN"),
        }
    }
}

/// fr fr Window function frame specification
#[derive(Debug, Clone)]
pub struct WindowFrame {
    pub frame_type: FrameType,
    pub start_bound: FrameBound,
    pub end_bound: Option<FrameBound>,
}

#[derive(Debug, Clone)]
pub enum FrameType {
    Rows,
    Range,
}

#[derive(Debug, Clone)]
pub enum FrameBound {
    UnboundedPreceding,
    Preceding(u64),
    CurrentRow,
    Following(u64),
    UnboundedFollowing,
}

/// fr fr Advanced SELECT query builder with complex features periodt
#[derive(Debug, Clone)]
pub struct AdvancedSelectBuilder {
    /// Selected columns and expressions
    select_fields: Vec<String>,
    /// FROM table or subquery
    from_clause: Option<String>,
    /// JOIN clauses
    joins: Vec<JoinClause>,
    /// WHERE conditions
    where_conditions: Vec<String>,
    /// GROUP BY fields
    group_by: Vec<String>,
    /// HAVING conditions
    having_conditions: Vec<String>,
    /// ORDER BY clauses
    order_by: Vec<OrderByClause>,
    /// LIMIT clause
    limit_clause: Option<u64>,
    /// OFFSET clause
    offset_clause: Option<u64>,
    /// Window functions
    window_functions: Vec<WindowFunction>,
    /// CASE expressions
    case_expressions: Vec<CaseExpression>,
    /// Subqueries
    subqueries: Vec<SubqueryClause>,
    /// Union/Intersect operations
    set_operations: Vec<SetOperation>,
    /// Query parameters
    parameters: Vec<SqlValue>,
}

#[derive(Debug, Clone)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: String,
    pub on_condition: String,
}

#[derive(Debug, Clone)]
pub struct OrderByClause {
    pub column: String,
    pub direction: OrderDirection,
    pub nulls_order: Option<NullsOrder>,
}

#[derive(Debug, Clone)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub enum NullsOrder {
    First,
    Last,
}

#[derive(Debug, Clone)]
pub struct WindowFunction {
    pub function_name: String,
    pub arguments: Vec<String>,
    pub partition_by: Vec<String>,
    pub order_by: Vec<OrderByClause>,
    pub frame: Option<WindowFrame>,
}

#[derive(Debug, Clone)]
pub struct CaseExpression {
    pub alias: String,
    pub when_clauses: Vec<WhenClause>,
    pub else_clause: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WhenClause {
    pub condition: String,
    pub result: String,
}

#[derive(Debug, Clone)]
pub struct SubqueryClause {
    pub alias: String,
    pub query: Box<AdvancedSelectBuilder>,
}

#[derive(Debug, Clone)]
pub struct SetOperation {
    pub operation_type: SetOperationType,
    pub query: Box<AdvancedSelectBuilder>,
    pub all: bool, // For UNION ALL, INTERSECT ALL, etc.
}

#[derive(Debug, Clone)]
pub enum SetOperationType {
    Union,
    Intersect,
    Except,
}

impl AdvancedSelectBuilder {
    /// slay Create new advanced SELECT builder
    #[instrument]
    pub fn new() -> Self {
        debug!("Creating new AdvancedSelectBuilder");
        Self {
            select_fields: Vec::new(),
            from_clause: None,
            joins: Vec::new(),
            where_conditions: Vec::new(),
            group_by: Vec::new(),
            having_conditions: Vec::new(),
            order_by: Vec::new(),
            limit_clause: None,
            offset_clause: None,
            window_functions: Vec::new(),
            case_expressions: Vec::new(),
            subqueries: Vec::new(),
            set_operations: Vec::new(),
            parameters: Vec::new(),
        }
    }

    /// facts Select specific columns or expressions
    #[instrument(skip(self))]
    pub fn select(mut self, fields: &[&str]) -> Self {
        debug!(fields = ?fields, "Adding select fields");
        self.select_fields.extend(fields.iter().map(|f| f.to_string()));
        self
    }

    /// highkey Select with aliases and complex expressions
    #[instrument(skip(self))]
    pub fn select_with_alias(mut self, expression: &str, alias: &str) -> Self {
        debug!(expression = expression, alias = alias, "Adding select field with alias");
        self.select_fields.push(format!("{} AS {}", expression, alias));
        self
    }

    /// sus Set FROM table or subquery
    #[instrument(skip(self))]
    pub fn from(mut self, table: &str) -> Self {
        debug!(table = table, "Setting FROM clause");
        self.from_clause = Some(table.to_string());
        self
    }

    /// lowkey Add INNER JOIN
    #[instrument(skip(self))]
    pub fn inner_join(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding INNER JOIN");
        self.joins.push(JoinClause {
            join_type: JoinType::Inner,
            table: table.to_string(),
            on_condition: on_condition.to_string(),
        });
        self
    }

    /// lowkey Add LEFT JOIN
    #[instrument(skip(self))]
    pub fn left_join(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding LEFT JOIN");
        self.joins.push(JoinClause {
            join_type: JoinType::Left,
            table: table.to_string(),
            on_condition: on_condition.to_string(),
        });
        self
    }

    /// lowkey Add RIGHT JOIN
    #[instrument(skip(self))]
    pub fn right_join(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding RIGHT JOIN");
        self.joins.push(JoinClause {
            join_type: JoinType::Right,
            table: table.to_string(),
            on_condition: on_condition.to_string(),
        });
        self
    }

    /// lowkey Add FULL OUTER JOIN
    #[instrument(skip(self))]
    pub fn full_outer_join(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding FULL OUTER JOIN");
        self.joins.push(JoinClause {
            join_type: JoinType::FullOuter,
            table: table.to_string(),
            on_condition: on_condition.to_string(),
        });
        self
    }

    /// yolo Add subquery in FROM clause
    #[instrument(skip(self, subquery))]
    pub fn from_subquery(mut self, subquery: AdvancedSelectBuilder, alias: &str) -> Self {
        debug!(alias = alias, "Adding subquery to FROM clause");
        self.subqueries.push(SubqueryClause {
            alias: alias.to_string(),
            query: Box::new(subquery),
        });
        self
    }

    /// facts Add window function
    #[instrument(skip(self))]
    pub fn window_function(
        mut self,
        function_name: &str,
        arguments: &[&str],
        partition_by: &[&str],
        order_by: &[(String, OrderDirection)],
    ) -> Self {
        debug!(
            function = function_name,
            args = ?arguments,
            partition = ?partition_by,
            "Adding window function"
        );
        
        let order_clauses = order_by
            .iter()
            .map(|(col, dir)| OrderByClause {
                column: col.clone(),
                direction: dir.clone(),
                nulls_order: None,
            })
            .collect();

        self.window_functions.push(WindowFunction {
            function_name: function_name.to_string(),
            arguments: arguments.iter().map(|a| a.to_string()).collect(),
            partition_by: partition_by.iter().map(|p| p.to_string()).collect(),
            order_by: order_clauses,
            frame: None,
        });
        self
    }

    /// bestie Add CASE expression
    #[instrument(skip(self))]
    pub fn case_expression(mut self, alias: &str) -> CaseExpressionBuilder {
        debug!(alias = alias, "Starting CASE expression");
        CaseExpressionBuilder::new(self, alias.to_string())
    }

    /// periodt Add UNION operation
    #[instrument(skip(self, other_query))]
    pub fn union(mut self, other_query: AdvancedSelectBuilder, all: bool) -> Self {
        debug!(all = all, "Adding UNION operation");
        self.set_operations.push(SetOperation {
            operation_type: SetOperationType::Union,
            query: Box::new(other_query),
            all,
        });
        self
    }

    /// periodt Add INTERSECT operation
    #[instrument(skip(self, other_query))]
    pub fn intersect(mut self, other_query: AdvancedSelectBuilder, all: bool) -> Self {
        debug!(all = all, "Adding INTERSECT operation");
        self.set_operations.push(SetOperation {
            operation_type: SetOperationType::Intersect,
            query: Box::new(other_query),
            all,
        });
        self
    }

    /// periodt Add EXCEPT operation
    #[instrument(skip(self, other_query))]
    pub fn except(mut self, other_query: AdvancedSelectBuilder, all: bool) -> Self {
        debug!(all = all, "Adding EXCEPT operation");
        self.set_operations.push(SetOperation {
            operation_type: SetOperationType::Except,
            query: Box::new(other_query),
            all,
        });
        self
    }

    /// sus Add WHERE condition
    #[instrument(skip(self))]
    pub fn where_condition(mut self, condition: &str) -> Self {
        debug!(condition = condition, "Adding WHERE condition");
        self.where_conditions.push(condition.to_string());
        self
    }

    /// facts Add parameterized WHERE condition
    #[instrument(skip(self))]
    pub fn where_param(mut self, condition: &str, value: SqlValue) -> Self {
        debug!(condition = condition, "Adding parameterized WHERE condition");
        self.where_conditions.push(condition.to_string());
        self.parameters.push(value);
        self
    }

    /// lowkey Add GROUP BY field
    #[instrument(skip(self))]
    pub fn group_by(mut self, field: &str) -> Self {
        debug!(field = field, "Adding GROUP BY field");
        self.group_by.push(field.to_string());
        self
    }

    /// highkey Add HAVING condition
    #[instrument(skip(self))]
    pub fn having(mut self, condition: &str) -> Self {
        debug!(condition = condition, "Adding HAVING condition");
        self.having_conditions.push(condition.to_string());
        self
    }

    /// slay Add ORDER BY clause
    #[instrument(skip(self))]
    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        debug!(column = column, direction = ?direction, "Adding ORDER BY clause");
        self.order_by.push(OrderByClause {
            column: column.to_string(),
            direction,
            nulls_order: None,
        });
        self
    }

    /// periodt Set LIMIT
    #[instrument(skip(self))]
    pub fn limit(mut self, limit: u64) -> Self {
        debug!(limit = limit, "Setting LIMIT");
        self.limit_clause = Some(limit);
        self
    }

    /// periodt Set OFFSET
    #[instrument(skip(self))]
    pub fn offset(mut self, offset: u64) -> Self {
        debug!(offset = offset, "Setting OFFSET");
        self.offset_clause = Some(offset);
        self
    }

    /// yolo Build the final query with optimizations
    #[instrument(skip(self))]
    pub fn build_optimized(&self) -> crate::error::Result<()> {
        info!("Building optimized advanced SQL query");
        
        let mut query_parts = Vec::new();
        
        // SELECT clause
        if self.select_fields.is_empty() {
            return Err(DatabaseError::query_error("No fields selected - what are we even querying bestie?"));
        }
        
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
        }
        
        // JOIN clauses
        for join in &self.joins {
            query_parts.push(format!("{} {} ON {}", join.join_type, join.table, join.on_condition));
        }
        
        // WHERE clause
        if !self.where_conditions.is_empty() {
            query_parts.push(format!("WHERE {}", self.where_conditions.join(" AND ")));
        }
        
        // GROUP BY clause
        if !self.group_by.is_empty() {
            query_parts.push(format!("GROUP BY {}", self.group_by.join(", ")));
        }
        
        // HAVING clause
        if !self.having_conditions.is_empty() {
            query_parts.push(format!("HAVING {}", self.having_conditions.join(" AND ")));
        }
        
        // ORDER BY clause
        if !self.order_by.is_empty() {
            let order_items: Vec<String> = self.order_by
                .iter()
                .map(|order| {
                    let direction = match order.direction {
                        OrderDirection::Asc => "ASC",
                        OrderDirection::Desc => "DESC",
                    };
                    format!("{} {}", order.column, direction)
                })
                .collect();
            query_parts.push(format!("ORDER BY {}", order_items.join(", ")));
        }
        
        // LIMIT clause
        if let Some(limit) = self.limit_clause {
            query_parts.push(format!("LIMIT {}", limit));
        }
        
        // OFFSET clause
        if let Some(offset) = self.offset_clause {
            query_parts.push(format!("OFFSET {}", offset));
        }
        
        // Set operations (UNION, INTERSECT, EXCEPT)
        let mut final_query = query_parts.join(" ");
        for set_op in &self.set_operations {
            let (other_query, _) = set_op.query.build_optimized()?;
            let operation = match set_op.operation_type {
                SetOperationType::Union => if set_op.all { "UNION ALL" } else { "UNION" },
                SetOperationType::Intersect => if set_op.all { "INTERSECT ALL" } else { "INTERSECT" },
                SetOperationType::Except => if set_op.all { "EXCEPT ALL" } else { "EXCEPT" },
            };
            final_query = format!("{} {} {}", final_query, operation, other_query);
        }
        
        trace!(query = %final_query, params = ?self.parameters, "Built advanced SQL query");
        Ok((final_query, self.parameters.clone()))
    }
}

impl QueryBuilder for AdvancedSelectBuilder {
    #[instrument(skip(self))]
    fn build(&self) -> crate::error::Result<()> {
        let (query, _) = self.build_optimized()?;
        Ok(query)
    }

    fn get_parameters(&self) -> Vec<SqlValue> {
        self.parameters.clone()
    }

    fn validate(&self) -> crate::error::Result<()> {
        if self.select_fields.is_empty() {
            return Err(DatabaseError::query_error("SELECT clause cannot be empty"));
        }
        
        if self.from_clause.is_none() && self.subqueries.is_empty() {
            return Err(DatabaseError::query_error("FROM clause is required"));
        }
        
        Ok(())
    }

    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr CASE expression builder for fluent SQL building
pub struct CaseExpressionBuilder {
    parent: AdvancedSelectBuilder,
    alias: String,
    when_clauses: Vec<WhenClause>,
    else_clause: Option<String>,
}

impl CaseExpressionBuilder {
    fn new(parent: AdvancedSelectBuilder, alias: String) -> Self {
        Self {
            parent,
            alias,
            when_clauses: Vec::new(),
            else_clause: None,
        }
    }

    /// yolo Add WHEN condition
    #[instrument(skip(self))]
    pub fn when(mut self, condition: &str, result: &str) -> Self {
        debug!(condition = condition, result = result, "Adding WHEN clause");
        self.when_clauses.push(WhenClause {
            condition: condition.to_string(),
            result: result.to_string(),
        });
        self
    }

    /// periodt Add ELSE clause
    #[instrument(skip(self))]
    pub fn else_clause(mut self, result: &str) -> Self {
        debug!(result = result, "Adding ELSE clause");
        self.else_clause = Some(result.to_string());
        self
    }

    /// facts Finish building CASE expression
    #[instrument(skip(self))]
    pub fn end_case(mut self) -> AdvancedSelectBuilder {
        debug!(alias = %self.alias, "Completing CASE expression");
        
        let case_expr = CaseExpression {
            alias: self.alias.clone(),
            when_clauses: self.when_clauses,
            else_clause: self.else_clause,
        };
        
        self.parent.case_expressions.push(case_expr);
        
        // Add to SELECT fields
        let case_sql = self.build_case_sql();
        self.parent.select_fields.push(format!("{} AS {}", case_sql, self.alias));
        
        self.parent
    }

    fn build_case_sql(&self) -> String {
        let mut case_parts = Vec::from(["CASE".to_string()]);
        
        for when_clause in &self.when_clauses {
            case_parts.push(format!("WHEN {} THEN {}", when_clause.condition, when_clause.result));
        }
        
        if let Some(ref else_result) = self.else_clause {
            case_parts.push(format!("ELSE {}", else_result));
        }
        
        case_parts.push("END".to_string());
        case_parts.join(" ")
    }
}

impl Default for AdvancedSelectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

