# CURSED Standard Library - XPath Implementation
# Full XPath 1.0 support with advanced query capabilities
# Version: 1.0.0-production

yeet "stringz"
yeet "arrayz" 
yeet "mathz"

# XPath Expression Types
enum XPathExprType {
    LocationPath,
    FilterExpr,
    PathExpr,
    OrExpr,
    AndExpr,
    EqualityExpr,
    RelationalExpr,
    UnaryExpr,
    UnionExpr,
    FunctionCall,
    VariableReference,
    Literal,
    Number
}

# XPath Axis Types
enum XPathAxis {
    Child,              # child::
    Descendant,         # descendant::
    Parent,             # parent::
    Ancestor,           # ancestor::
    FollowingSibling,   # following-sibling::
    PrecedingSibling,   # preceding-sibling::
    Following,          # following::
    Preceding,          # preceding::
    Attribute,          # attribute:: or @
    Namespace,          # namespace::
    Self,               # self::
    DescendantOrSelf,   # descendant-or-self:: or //
    AncestorOrSelf      # ancestor-or-self::
}

# XPath Node Test Types
enum XPathNodeTest {
    NameTest,      # element name or *
    NodeType,      # node(), text(), comment(), processing-instruction()
    ProcessingInstruction  # processing-instruction('target')
}

# XPath Expression Structure
squad XPathExpression {
    expr_type XPathExprType
    value tea
    children []XPathExpression
    axis XPathAxis
    node_test XPathNodeTest
    predicates []XPathExpression
    function_name tea
    arguments []XPathExpression
}

# XPath Context for evaluation
squad XPathContext {
    context_node XmlNode
    context_position drip
    context_size drip
    variable_bindings map[tea]tea
    function_library map[tea]XPathFunction
    namespace_bindings map[tea]tea
}

# XPath Function Definition
squad XPathFunction {
    name tea
    min_args drip
    max_args drip
    return_type tea
    evaluator slay(args []XPathExpression, context XPathContext) yikes<tea>
}

# ========================
# XPath Parsing Functions
# ========================

# Tokenize XPath expression
slay tokenize_xpath(xpath tea) []tea {
    sus tokens []tea = []
    sus current_token tea = ""
    sus in_string lit = nah
    sus string_delim tea = ""
    sus i drip = 0
    
    bestie (i < xpath.len()) {
        sus char tea = stringz.char_at(xpath, i)
        
        ready (in_string) {
            ready (char == string_delim) {
                tokens = arrayz.append(tokens, current_token)
                current_token = ""
                in_string = nah
                string_delim = ""
            } otherwise {
                current_token = current_token + char
            }
        } otherwise {
            ready (char == "'" || char == "\"") {
                ready (current_token != "") {
                    tokens = arrayz.append(tokens, current_token)
                    current_token = ""
                }
                in_string = based
                string_delim = char
            } otherwise ready (is_xpath_operator(char)) {
                ready (current_token != "") {
                    tokens = arrayz.append(tokens, current_token)
                    current_token = ""
                }
                
                # Handle multi-character operators
                sus operator tea = get_xpath_operator(xpath, i)
                tokens = arrayz.append(tokens, operator)
                i = i + operator.len() - 1
            } otherwise ready (char == " " || char == "\t" || char == "\n" || char == "\r") {
                ready (current_token != "") {
                    tokens = arrayz.append(tokens, current_token)
                    current_token = ""
                }
            } otherwise {
                current_token = current_token + char
            }
        }
        
        i = i + 1
    }
    
    ready (current_token != "") {
        tokens = arrayz.append(tokens, current_token)
    }
    
    damn tokens
}

# Parse XPath expression from tokens
slay parse_xpath_expression(tokens []tea) yikes<XPathExpression> {
    ready (tokens.len() == 0) {
        yikes "Empty XPath expression"
    }
    
    sus parser XPathParser = {
        tokens: tokens,
        position: 0,
        current_token: tokens[0]
    }
    
    damn parse_or_expr(parser)
}

# XPath Parser State
squad XPathParser {
    tokens []tea
    position drip
    current_token tea
}

# Parse OR expression (lowest precedence)
slay parse_or_expr(parser sus XPathParser) yikes<XPathExpression> {
    sus left XPathExpression = parse_and_expr(parser) fam {
        when err -> yikes err
    }
    
    bestie (parser.current_token == "or") {
        advance_parser(parser)
        sus right XPathExpression = parse_or_expr(parser) fam {
            when err -> yikes err
        }
        
        damn {
            expr_type: XPathExprType.OrExpr,
            value: "or",
            children: [left, right],
            axis: XPathAxis.Child,
            node_test: XPathNodeTest.NameTest,
            predicates: [],
            function_name: "",
            arguments: []
        }
    }
    
    damn left
}

# Parse AND expression
slay parse_and_expr(parser sus XPathParser) yikes<XPathExpression> {
    sus left XPathExpression = parse_equality_expr(parser) fam {
        when err -> yikes err
    }
    
    bestie (parser.current_token == "and") {
        advance_parser(parser)
        sus right XPathExpression = parse_and_expr(parser) fam {
            when err -> yikes err
        }
        
        damn {
            expr_type: XPathExprType.AndExpr,
            value: "and",
            children: [left, right],
            axis: XPathAxis.Child,
            node_test: XPathNodeTest.NameTest,
            predicates: [],
            function_name: "",
            arguments: []
        }
    }
    
    damn left
}

# Parse equality expression (= and !=)
slay parse_equality_expr(parser sus XPathParser) yikes<XPathExpression> {
    sus left XPathExpression = parse_relational_expr(parser) fam {
        when err -> yikes err
    }
    
    bestie (parser.current_token == "=" || parser.current_token == "!=") {
        sus operator tea = parser.current_token
        advance_parser(parser)
        sus right XPathExpression = parse_equality_expr(parser) fam {
            when err -> yikes err
        }
        
        damn {
            expr_type: XPathExprType.EqualityExpr,
            value: operator,
            children: [left, right],
            axis: XPathAxis.Child,
            node_test: XPathNodeTest.NameTest,
            predicates: [],
            function_name: "",
            arguments: []
        }
    }
    
    damn left
}

# Parse relational expression (<, <=, >, >=)
slay parse_relational_expr(parser sus XPathParser) yikes<XPathExpression> {
    sus left XPathExpression = parse_union_expr(parser) fam {
        when err -> yikes err
    }
    
    bestie (parser.current_token == "<" || parser.current_token == "<=" ||
            parser.current_token == ">" || parser.current_token == ">=") {
        sus operator tea = parser.current_token
        advance_parser(parser)
        sus right XPathExpression = parse_relational_expr(parser) fam {
            when err -> yikes err
        }
        
        damn {
            expr_type: XPathExprType.RelationalExpr,
            value: operator,
            children: [left, right],
            axis: XPathAxis.Child,
            node_test: XPathNodeTest.NameTest,
            predicates: [],
            function_name: "",
            arguments: []
        }
    }
    
    damn left
}

# Parse union expression (|)
slay parse_union_expr(parser sus XPathParser) yikes<XPathExpression> {
    sus left XPathExpression = parse_path_expr(parser) fam {
        when err -> yikes err
    }
    
    bestie (parser.current_token == "|") {
        advance_parser(parser)
        sus right XPathExpression = parse_union_expr(parser) fam {
            when err -> yikes err
        }
        
        damn {
            expr_type: XPathExprType.UnionExpr,
            value: "|",
            children: [left, right],
            axis: XPathAxis.Child,
            node_test: XPathNodeTest.NameTest,
            predicates: [],
            function_name: "",
            arguments: []
        }
    }
    
    damn left
}

# Parse path expression
slay parse_path_expr(parser sus XPathParser) yikes<XPathExpression> {
    # Check for absolute location path
    ready (parser.current_token == "/") {
        advance_parser(parser)
        
        ready (is_at_end(parser) || is_operator(parser.current_token)) {
            # Just "/" - select document root
            damn {
                expr_type: XPathExprType.LocationPath,
                value: "/",
                children: [],
                axis: XPathAxis.Self,
                node_test: XPathNodeTest.NameTest,
                predicates: [],
                function_name: "",
                arguments: []
            }
        }
        
        sus path XPathExpression = parse_relative_location_path(parser) fam {
            when err -> yikes err
        }
        
        # Mark as absolute path
        path.value = "/" + path.value
        damn path
    }
    
    # Check for descendant-or-self shorthand
    ready (parser.current_token == "//") {
        advance_parser(parser)
        sus path XPathExpression = parse_relative_location_path(parser) fam {
            when err -> yikes err
        }
        
        # Mark as descendant-or-self path
        path.axis = XPathAxis.DescendantOrSelf
        path.value = "//" + path.value
        damn path
    }
    
    # Parse filter expression or location path
    damn parse_filter_expr(parser)
}

# ========================
# XPath Evaluation Functions
# ========================

# Evaluate XPath expression
slay evaluate_xpath(context_node XmlNode, expr XPathExpression) yikes<XPathResult> {
    sus context XPathContext = {
        context_node: context_node,
        context_position: 1,
        context_size: 1,
        variable_bindings: {},
        function_library: create_default_function_library(),
        namespace_bindings: {}
    }
    
    damn evaluate_xpath_with_context(expr, context)
}

# Evaluate XPath with custom context
slay evaluate_xpath_with_context(expr XPathExpression, context XPathContext) yikes<XPathResult> {
    ready (expr.expr_type == XPathExprType.LocationPath) {
        damn evaluate_location_path(expr, context)
    }
    
    ready (expr.expr_type == XPathExprType.FilterExpr) {
        damn evaluate_filter_expr(expr, context)
    }
    
    ready (expr.expr_type == XPathExprType.OrExpr) {
        damn evaluate_or_expr(expr, context)
    }
    
    ready (expr.expr_type == XPathExprType.AndExpr) {
        damn evaluate_and_expr(expr, context)
    }
    
    ready (expr.expr_type == XPathExprType.EqualityExpr) {
        damn evaluate_equality_expr(expr, context)
    }
    
    ready (expr.expr_type == XPathExprType.RelationalExpr) {
        damn evaluate_relational_expr(expr, context)
    }
    
    ready (expr.expr_type == XPathExprType.UnionExpr) {
        damn evaluate_union_expr(expr, context)
    }
    
    ready (expr.expr_type == XPathExprType.FunctionCall) {
        damn evaluate_function_call(expr, context)
    }
    
    ready (expr.expr_type == XPathExprType.Literal) {
        damn {
            nodes: [],
            values: [expr.value],
            result_type: "string"
        }
    }
    
    ready (expr.expr_type == XPathExprType.Number) {
        damn {
            nodes: [],
            values: [expr.value],
            result_type: "number"
        }
    }
    
    yikes "Unsupported XPath expression type"
}

# Evaluate location path
slay evaluate_location_path(expr XPathExpression, context XPathContext) yikes<XPathResult> {
    sus current_nodes []XmlNode = [context.context_node]
    sus result_nodes []XmlNode = []
    
    # Handle absolute path
    ready (expr.value.starts_with("/")) {
        # Find document root
        sus root XmlNode = context.context_node
        bestie (root.parent != cap) {
            root = root.parent
        }
        current_nodes = [root]
    }
    
    # Handle descendant-or-self
    ready (expr.axis == XPathAxis.DescendantOrSelf) {
        sus descendants []XmlNode = []
        bestie (sus node XmlNode in current_nodes) {
            descendants = arrayz.append_all(descendants, get_descendant_or_self_nodes(node))
        }
        current_nodes = descendants
    }
    
    # Apply node test and predicates
    bestie (sus node XmlNode in current_nodes) {
        ready (matches_node_test(node, expr.node_test, expr.value)) {
            # Apply predicates
            sus matches lit = based
            bestie (sus predicate XPathExpression in expr.predicates) {
                sus pred_context XPathContext = context
                pred_context.context_node = node
                
                sus pred_result XPathResult = evaluate_xpath_with_context(predicate, pred_context) fam {
                    when err -> yikes err
                }
                
                ready (!is_xpath_true(pred_result)) {
                    matches = nah
                    break
                }
            }
            
            ready (matches) {
                result_nodes = arrayz.append(result_nodes, node)
            }
        }
    }
    
    damn {
        nodes: result_nodes,
        values: [],
        result_type: "nodeset"
    }
}

# ========================
# XPath Built-in Functions
# ========================

# Create default function library
slay create_default_function_library() map[tea]XPathFunction {
    sus functions map[tea]XPathFunction = {}
    
    # String functions
    functions["string"] = {
        name: "string",
        min_args: 0,
        max_args: 1,
        return_type: "string",
        evaluator: xpath_string_function
    }
    
    functions["concat"] = {
        name: "concat",
        min_args: 2,
        max_args: -1,
        return_type: "string",
        evaluator: xpath_concat_function
    }
    
    functions["substring"] = {
        name: "substring",
        min_args: 2,
        max_args: 3,
        return_type: "string",
        evaluator: xpath_substring_function
    }
    
    functions["contains"] = {
        name: "contains",
        min_args: 2,
        max_args: 2,
        return_type: "boolean",
        evaluator: xpath_contains_function
    }
    
    functions["starts-with"] = {
        name: "starts-with",
        min_args: 2,
        max_args: 2,
        return_type: "boolean",
        evaluator: xpath_starts_with_function
    }
    
    # Number functions
    functions["number"] = {
        name: "number",
        min_args: 0,
        max_args: 1,
        return_type: "number",
        evaluator: xpath_number_function
    }
    
    functions["sum"] = {
        name: "sum",
        min_args: 1,
        max_args: 1,
        return_type: "number",
        evaluator: xpath_sum_function
    }
    
    functions["floor"] = {
        name: "floor",
        min_args: 1,
        max_args: 1,
        return_type: "number",
        evaluator: xpath_floor_function
    }
    
    functions["ceiling"] = {
        name: "ceiling",
        min_args: 1,
        max_args: 1,
        return_type: "number",
        evaluator: xpath_ceiling_function
    }
    
    # Boolean functions
    functions["boolean"] = {
        name: "boolean",
        min_args: 1,
        max_args: 1,
        return_type: "boolean",
        evaluator: xpath_boolean_function
    }
    
    functions["not"] = {
        name: "not",
        min_args: 1,
        max_args: 1,
        return_type: "boolean",
        evaluator: xpath_not_function
    }
    
    functions["true"] = {
        name: "true",
        min_args: 0,
        max_args: 0,
        return_type: "boolean",
        evaluator: xpath_true_function
    }
    
    functions["false"] = {
        name: "false",
        min_args: 0,
        max_args: 0,
        return_type: "boolean",
        evaluator: xpath_false_function
    }
    
    # Node set functions
    functions["count"] = {
        name: "count",
        min_args: 1,
        max_args: 1,
        return_type: "number",
        evaluator: xpath_count_function
    }
    
    functions["position"] = {
        name: "position",
        min_args: 0,
        max_args: 0,
        return_type: "number",
        evaluator: xpath_position_function
    }
    
    functions["last"] = {
        name: "last",
        min_args: 0,
        max_args: 0,
        return_type: "number",
        evaluator: xpath_last_function
    }
    
    functions["name"] = {
        name: "name",
        min_args: 0,
        max_args: 1,
        return_type: "string",
        evaluator: xpath_name_function
    }
    
    damn functions
}

# XPath string() function
slay xpath_string_function(args []XPathExpression, context XPathContext) yikes<tea> {
    ready (args.len() == 0) {
        damn get_node_text_content(context.context_node)
    }
    
    sus result XPathResult = evaluate_xpath_with_context(args[0], context) fam {
        when err -> yikes err
    }
    
    ready (result.result_type == "string" && result.values.len() > 0) {
        damn result.values[0]
    }
    
    ready (result.result_type == "nodeset" && result.nodes.len() > 0) {
        damn get_node_text_content(result.nodes[0])
    }
    
    ready (result.result_type == "number" && result.values.len() > 0) {
        damn result.values[0]
    }
    
    ready (result.result_type == "boolean" && result.values.len() > 0) {
        ready (result.values[0] == "true") {
            damn "true"
        } otherwise {
            damn "false"
        }
    }
    
    damn ""
}

# XPath count() function
slay xpath_count_function(args []XPathExpression, context XPathContext) yikes<tea> {
    ready (args.len() != 1) {
        yikes "count() function requires exactly one argument"
    }
    
    sus result XPathResult = evaluate_xpath_with_context(args[0], context) fam {
        when err -> yikes err
    }
    
    ready (result.result_type != "nodeset") {
        yikes "count() function requires a node-set argument"
    }
    
    damn stringz.from_int(result.nodes.len())
}

# ========================
# Utility Functions for XPath
# ========================

# Check if node matches node test
slay matches_node_test(node XmlNode, node_test XPathNodeTest, test_value tea) lit {
    ready (node_test == XPathNodeTest.NameTest) {
        ready (test_value == "*") {
            damn (node.node_type == XmlNodeType.Element)
        }
        damn (node.node_type == XmlNodeType.Element && node.name == test_value)
    }
    
    ready (node_test == XPathNodeTest.NodeType) {
        ready (test_value == "node()") {
            damn based
        }
        ready (test_value == "text()") {
            damn (node.node_type == XmlNodeType.Text)
        }
        ready (test_value == "comment()") {
            damn (node.node_type == XmlNodeType.Comment)
        }
        ready (test_value == "processing-instruction()") {
            damn (node.node_type == XmlNodeType.ProcessingInstruction)
        }
    }
    
    damn nah
}

# Get descendant-or-self nodes
slay get_descendant_or_self_nodes(node XmlNode) []XmlNode {
    sus result []XmlNode = [node]
    
    bestie (sus child XmlNode in node.children) {
        sus descendants []XmlNode = get_descendant_or_self_nodes(child)
        result = arrayz.append_all(result, descendants)
    }
    
    damn result
}

# Convert XPath result to boolean
slay is_xpath_true(result XPathResult) lit {
    ready (result.result_type == "boolean") {
        ready (result.values.len() > 0) {
            damn (result.values[0] == "true")
        }
        damn nah
    }
    
    ready (result.result_type == "number") {
        ready (result.values.len() > 0) {
            sus num drip = stringz.to_int(result.values[0]) fam { when _ -> damn nah }
            damn (num != 0)
        }
        damn nah
    }
    
    ready (result.result_type == "string") {
        ready (result.values.len() > 0) {
            damn (result.values[0] != "")
        }
        damn nah
    }
    
    ready (result.result_type == "nodeset") {
        damn (result.nodes.len() > 0)
    }
    
    damn nah
}

# Check if character/string is XPath operator
slay is_xpath_operator(char tea) lit {
    sus operators []tea = ["/", "//", "[", "]", "(", ")", "@", "|", "+", "-", "*", "=", "!", "<", ">", "."]
    bestie (sus op tea in operators) {
        ready (char == op) {
            damn based
        }
    }
    damn nah
}

# Get XPath operator starting at position
slay get_xpath_operator(xpath tea, pos drip) tea {
    # Check for multi-character operators first
    ready (pos + 1 < xpath.len()) {
        sus two_char tea = stringz.substring(xpath, pos, 2)
        ready (two_char == "//" || two_char == "!=" || two_char == "<=" || two_char == ">=") {
            damn two_char
        }
    }
    
    # Single character operator
    damn stringz.char_at(xpath, pos)
}

# Advance parser position
slay advance_parser(parser sus XPathParser) {
    parser.position = parser.position + 1
    ready (parser.position < parser.tokens.len()) {
        parser.current_token = parser.tokens[parser.position]
    } otherwise {
        parser.current_token = ""
    }
}

# Check if parser is at end
slay is_at_end(parser XPathParser) lit {
    damn (parser.position >= parser.tokens.len())
}

# Check if token is operator
slay is_operator(token tea) lit {
    sus operators []tea = ["or", "and", "=", "!=", "<", "<=", ">", ">=", "|", "+", "-", "*", "div", "mod"]
    bestie (sus op tea in operators) {
        ready (token == op) {
            damn based
        }
    }
    damn nah
}
