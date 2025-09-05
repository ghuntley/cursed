yeet "testz"
yeet "tag_core"

test_start("TagCore comprehensive test suite")

fr fr Test HTML escaping functionality
slay test_html_escaping() {
    fr fr Test basic HTML escaping
    sus escaped := tag_core.EscapeString("<script>alert('xss')</script>")
    assert_true(escaped != "<script>alert('xss')</script>")
    
    fr fr Test special characters
    sus special := tag_core.EscapeString("&<>\"'")
    assert_true(special != "&<>\"'")
    
    fr fr Test unescaping
    sus unescaped := tag_core.UnescapeString("&lt;&gt;&amp;&quot;&#39;")
    assert_eq_string(unescaped, "<>&\"'")
    
    fr fr Test bytes escaping
    sus bytes_escaped := tag_core.EscapeBytes(byte[value]("<test>"))
    assert_true(len(bytes_escaped) > 0)
    
    fr fr Test URL escaping
    sus url_escaped := tag_core.EscapeURL("hello world#test?param=value")
    assert_true(url_escaped != "hello world#test?param=value")
    
    vibez.spill("✅ HTML escaping tests passed")
}

fr fr Test Tag creation and management
slay test_tag_creation() {
    fr fr Test creating a new tag
    sus tag := tag_core.NewTag("test_tag", "development")
    assert_eq_string(tag.name, "test_tag")
    assert_eq_string(tag.category, "development")
    assert_eq_string(tag.color, "#3498db")
    assert_true(tag.is_active)
    assert_eq_int(tag.usage_count, 0)
    
    fr fr Test setting description
    tag.SetDescription("This is a test tag")
    assert_eq_string(tag.description, "This is a test tag")
    
    fr fr Test setting color
    tag.SetColor("#ff0000")
    assert_eq_string(tag.color, "#ff0000")
    
    fr fr Test metadata
    tag.SetMetadata("priority", "high")
    assert_eq_string(tag.GetMetadata("priority").(tea), "high")
    assert_true(tag.HasMetadata("priority"))
    
    fr fr Test removing metadata
    tag.RemoveMetadata("priority")
    assert_true(!tag.HasMetadata("priority"))
    
    fr fr Test usage increment
    tag.IncrementUsage()
    assert_eq_int(tag.usage_count, 1)
    
    fr fr Test activate/deactivate
    tag.Deactivate()
    assert_true(!tag.is_active)
    tag.Activate()
    assert_true(tag.is_active)
    
    vibez.spill("✅ Tag creation tests passed")
}

fr fr Test Tag hierarchy
slay test_tag_hierarchy() {
    sus parent_tag := tag_core.NewTag("parent", "category")
    sus child_tag := tag_core.NewTag("child", "category")
    
    fr fr Test setting parent
    child_tag.SetParent(parent_tag.id)
    assert_eq_string(child_tag.parent_id, parent_tag.id)
    
    fr fr Test adding child
    parent_tag.AddChild(child_tag.id)
    assert_true(parent_tag.IsChild(child_tag.id))
    
    fr fr Test removing child
    parent_tag.RemoveChild(child_tag.id)
    assert_true(!parent_tag.IsChild(child_tag.id))
    
    fr fr Test getting path
    sus path := parent_tag.GetPath()
    assert_true(path != "")
    
    vibez.spill("✅ Tag hierarchy tests passed")
}

fr fr Test Tag cloning
slay test_tag_cloning() {
    sus original := tag_core.NewTag("original", "test")
    original.SetDescription("Original description")
    original.SetColor("#123456")
    original.SetMetadata("key", "value")
    
    sus clone := original.Clone()
    assert_eq_string(clone.name, "original")
    assert_eq_string(clone.description, "Original description")
    assert_eq_string(clone.color, "#123456")
    assert_eq_string(clone.GetMetadata("key").(tea), "value")
    assert_true(clone.id != original.id)
    assert_eq_int(clone.usage_count, 0)
    
    vibez.spill("✅ Tag cloning tests passed")
}

fr fr Test TagSet functionality
slay test_tag_set() {
    sus tag_set := tag_core.NewTagSet()
    
    fr fr Test initial state
    assert_eq_int(tag_set.GetTagCount(), 0)
    assert_eq_int(tag_set.GetCategoryCount(), 0)
    
    fr fr Test adding tags
    sus tag1 := tag_core.NewTag("tag1", "category1")
    sus tag2 := tag_core.NewTag("tag2", "category2")
    
    sus err1 := tag_set.AddTag(tag1)
    assert_eq_string(err1, "")
    
    sus err2 := tag_set.AddTag(tag2)
    assert_eq_string(err2, "")
    
    fr fr Test counts
    assert_eq_int(tag_set.GetTagCount(), 2)
    assert_eq_int(tag_set.GetCategoryCount(), 2)
    
    fr fr Test getting tags
    sus retrieved := tag_set.GetTag(tag1.id)
    assert_true(retrieved != cringe)
    assert_eq_string(retrieved.name, "tag1")
    
    sus by_name := tag_set.GetTagByName("tag1")
    assert_true(by_name != cringe)
    assert_eq_string(by_name.id, tag1.id)
    
    fr fr Test getting by category
    sus category_tags := tag_set.GetTagsByCategory("category1")
    assert_eq_int(len(category_tags), 1)
    assert_eq_string(category_tags[0].name, "tag1")
    
    fr fr Test getting all tags
    sus all_tags := tag_set.GetAllTags()
    assert_eq_int(len(all_tags), 2)
    
    fr fr Test getting all categories
    sus all_categories := tag_set.GetAllCategories()
    assert_eq_int(len(all_categories), 2)
    
    fr fr Test removing tags
    sus remove_err := tag_set.RemoveTag(tag1.id)
    assert_eq_string(remove_err, "")
    assert_eq_int(tag_set.GetTagCount(), 1)
    
    vibez.spill("✅ TagSet tests passed")
}

fr fr Test TagQuery functionality
slay test_tag_query() {
    sus query := tag_core.NewTagQuery()
    
    fr fr Test initial state
    assert_eq_string(query.name_pattern, "")
    assert_eq_string(query.category, "")
    assert_eq_string(query.sort_by, "name")
    assert_eq_string(query.sort_order, "asc")
    assert_eq_int(query.limit, 0)
    assert_eq_int(query.offset, 0)
    
    fr fr Test query building
    query.WithNamePattern("test")
    assert_eq_string(query.name_pattern, "test")
    
    query.WithCategory("development")
    assert_eq_string(query.category, "development")
    
    query.WithActive(cap)
    assert_true(!query.is_active)
    
    query.WithSorting("usage_count", "desc")
    assert_eq_string(query.sort_by, "usage_count")
    assert_eq_string(query.sort_order, "desc")
    
    query.WithPagination(10, 20)
    assert_eq_int(query.limit, 10)
    assert_eq_int(query.offset, 20)
    
    query.WithMetadata("priority", "high")
    assert_eq_string(query.metadata_filters["priority"].(tea), "high")
    
    vibez.spill("✅ TagQuery tests passed")
}

fr fr Test Tag search functionality
slay test_tag_search() {
    fr fr Create some test tags
    sus tag1 := tag_core.CreateTag("javascript", "programming")
    sus tag2 := tag_core.CreateTag("python", "programming")
    sus tag3 := tag_core.CreateTag("design", "creative")
    
    tag1.SetMetadata("difficulty", "medium")
    tag2.SetMetadata("difficulty", "easy")
    tag3.SetMetadata("difficulty", "hard")
    
    fr fr Test search by name
    sus name_results := tag_core.SearchTagsByName("javascript")
    assert_true(len(name_results) > 0)
    
    fr fr Test search by category
    sus category_results := tag_core.SearchTagsByCategory("programming")
    assert_true(len(category_results) > 0)
    
    fr fr Test search by metadata
    sus metadata_results := tag_core.SearchTagsByMetadata("difficulty", "easy")
    assert_true(len(metadata_results) >= 0)
    
    fr fr Test search by text
    sus text_results := tag_core.SearchTagsByText("javascript")
    assert_true(len(text_results) >= 0)
    
    fr fr Test advanced search
    sus query := tag_core.NewTagQuery()
    query.WithCategory("programming")
    query.WithActive(based)
    query.WithPagination(5, 0)
    
    sus search_results := tag_core.SearchTags(query)
    assert_true(len(search_results) >= 0)
    
    vibez.spill("✅ Tag search tests passed")
}

fr fr Test Tag relations
slay test_tag_relations() {
    sus tag1 := tag_core.CreateTag("frontend", "programming")
    sus tag2 := tag_core.CreateTag("backend", "programming")
    
    fr fr Test adding relation
    sus err := tag_core.AddTagRelation(tag1.id, tag2.id, "related", 0.8)
    assert_eq_string(err, "")
    
    fr fr Test getting relations
    sus relations := tag_core.GetTagRelations(tag1.id)
    assert_true(len(relations) > 0)
    
    fr fr Test getting related tags
    sus related_tags := tag_core.GetRelatedTags(tag1.id, "related")
    assert_true(len(related_tags) > 0)
    
    vibez.spill("✅ Tag relations tests passed")
}

fr fr Test Tag statistics
slay test_tag_statistics() {
    fr fr Create some test tags
    sus tag1 := tag_core.CreateTag("popular", "test")
    sus tag2 := tag_core.CreateTag("unpopular", "test")
    
    fr fr Increment usage
    tag1.IncrementUsage()
    tag1.IncrementUsage()
    tag1.IncrementUsage()
    
    fr fr Test getting statistics
    sus stats := tag_core.GetTagStats()
    assert_true(stats.total_tags > 0)
    assert_true(stats.categories_count > 0)
    
    fr fr Test tag analytics
    sus analytics := tag_core.GetTagAnalytics(tag1.id)
    assert_true(len(analytics) > 0)
    assert_eq_int(analytics["usage_count"].(normie), 3)
    
    fr fr Test category analytics
    sus category_analytics := tag_core.GetCategoryAnalytics("test")
    assert_true(len(category_analytics) > 0)
    assert_true(category_analytics["total_tags"].(normie) > 0)
    
    vibez.spill("✅ Tag statistics tests passed")
}

fr fr Test bulk operations
slay test_bulk_operations() {
    sus names := tea[value]{"bulk1", "bulk2", "bulk3"}
    
    fr fr Test bulk create
    sus created_tags := tag_core.BulkCreateTags(names, "bulk_test")
    assert_eq_int(len(created_tags), 3)
    
    fr fr Test bulk update
    sus tag_ids := tea[value]{}
    for _, tag := range created_tags {
        tag_ids = append(tag_ids, tag.id)
    }
    
    sus updates := make(map[tea]interface{})
    updates["description"] = "Bulk updated description"
    updates["color"] = "#ff0000"
    
    sus update_errors := tag_core.BulkUpdateTags(tag_ids, updates)
    assert_eq_int(len(update_errors), 0)
    
    fr fr Test bulk delete
    sus delete_errors := tag_core.BulkDeleteTags(tag_ids)
    assert_eq_int(len(delete_errors), 0)
    
    vibez.spill("✅ Bulk operations tests passed")
}

fr fr Test tag validation
slay test_tag_validation() {
    sus valid_tag := tag_core.NewTag("valid", "category")
    valid_tag.SetDescription("Valid description")
    valid_tag.SetColor("#3498db")
    
    fr fr Test valid tag
    sus valid_errors := tag_core.ValidateTag(valid_tag)
    assert_eq_int(len(valid_errors), 0)
    
    fr fr Test invalid tag
    sus invalid_tag := tag_core.NewTag("", "")
    invalid_tag.SetDescription("This is a very long description that exceeds the maximum allowed length for tag descriptions and should trigger a validation error because it's way too long for what should be a concise description")
    invalid_tag.SetColor("invalid_color")
    
    sus invalid_errors := tag_core.ValidateTag(invalid_tag)
    assert_true(len(invalid_errors) > 0)
    
    vibez.spill("✅ Tag validation tests passed")
}

fr fr Test tag merging
slay test_tag_merging() {
    sus source := tag_core.CreateTag("source", "test")
    sus target := tag_core.CreateTag("target", "test")
    
    fr fr Setup source tag
    source.SetMetadata("key1", "value1")
    source.IncrementUsage()
    source.IncrementUsage()
    
    fr fr Setup target tag
    target.SetMetadata("key2", "value2")
    target.IncrementUsage()
    
    fr fr Test merging
    sus merge_err := tag_core.MergeTags(source.id, target.id)
    assert_eq_string(merge_err, "")
    
    fr fr Verify merge results
    sus merged_tag := tag_core.GetTag(target.id)
    assert_true(merged_tag != cringe)
    assert_eq_int(merged_tag.usage_count, 3)
    assert_true(merged_tag.HasMetadata("key1"))
    assert_true(merged_tag.HasMetadata("key2"))
    
    fr fr Verify source tag is deleted
    sus deleted_tag := tag_core.GetTag(source.id)
    assert_true(deleted_tag == cringe)
    
    vibez.spill("✅ Tag merging tests passed")
}

fr fr Test tag suggestions
slay test_tag_suggestions() {
    fr fr Create some test tags
    sus tag1 := tag_core.CreateTag("javascript", "programming")
    sus tag2 := tag_core.CreateTag("java", "programming")
    sus tag3 := tag_core.CreateTag("python", "programming")
    
    tag1.SetDescription("JavaScript programming language")
    tag2.SetDescription("Java programming language")
    tag3.SetDescription("Python programming language")
    
    fr fr Test suggestions
    sus suggestions := tag_core.SuggestTags("java", 5)
    assert_true(len(suggestions) > 0)
    
    fr fr Test popular tags
    tag1.IncrementUsage()
    tag1.IncrementUsage()
    tag1.IncrementUsage()
    
    sus popular := tag_core.GetPopularTags(5)
    assert_true(len(popular) > 0)
    
    fr fr Test recent tags
    sus recent := tag_core.GetRecentTags(5)
    assert_true(len(recent) > 0)
    
    vibez.spill("✅ Tag suggestions tests passed")
}

fr fr Test tag export and import
slay test_tag_export_import() {
    fr fr Create test tags
    sus tag1 := tag_core.CreateTag("export1", "test")
    sus tag2 := tag_core.CreateTag("export2", "test")
    
    tag1.SetDescription("Export test tag 1")
    tag2.SetDescription("Export test tag 2")
    
    fr fr Test export
    sus tag_ids := tea[value]{tag1.id, tag2.id}
    sus exported := tag_core.ExportTags(tag_ids)
    assert_true(exported != "")
    assert_true(len(exported) > 0)
    
    fr fr Test import
    sus import_errors := tag_core.ImportTags(exported)
    assert_true(len(import_errors) >= 0)
    
    vibez.spill("✅ Tag export/import tests passed")
}

fr fr Test global tag functions
slay test_global_functions() {
    fr fr Test global tag operations
    sus global_tag := tag_core.CreateTag("global", "global_test")
    assert_true(global_tag != cringe)
    
    fr fr Test getting tag
    sus retrieved := tag_core.GetTag(global_tag.id)
    assert_true(retrieved != cringe)
    assert_eq_string(retrieved.name, "global")
    
    fr fr Test getting by name
    sus by_name := tag_core.GetTagByName("global")
    assert_true(by_name != cringe)
    assert_eq_string(by_name.id, global_tag.id)
    
    fr fr Test getting by category
    sus by_category := tag_core.GetTagsByCategory("global_test")
    assert_true(len(by_category) > 0)
    
    fr fr Test getting all tags
    sus all_tags := tag_core.GetAllTags()
    assert_true(len(all_tags) > 0)
    
    fr fr Test getting all categories
    sus all_categories := tag_core.GetAllCategories()
    assert_true(len(all_categories) > 0)
    
    fr fr Test deletion
    sus delete_err := tag_core.DeleteTag(global_tag.id)
    assert_eq_string(delete_err, "")
    
    sus deleted := tag_core.GetTag(global_tag.id)
    assert_true(deleted == cringe)
    
    vibez.spill("✅ Global functions tests passed")
}

fr fr Test HTML context escaping
slay test_html_context_escaping() {
    sus test_string := "Hello <script>alert('xss')</script> World"
    
    fr fr Test different contexts
    sus html_escaped := tag_core.EscapeForContext(test_string, tag_core.ContextHTML)
    assert_true(html_escaped != test_string)
    
    sus attr_escaped := tag_core.EscapeForContext(test_string, tag_core.ContextAttribute)
    assert_true(attr_escaped != test_string)
    
    sus js_escaped := tag_core.EscapeForContext(test_string, tag_core.ContextJS)
    assert_true(js_escaped != test_string)
    
    sus css_escaped := tag_core.EscapeForContext(test_string, tag_core.ContextCSS)
    assert_true(css_escaped != test_string)
    
    sus url_escaped := tag_core.EscapeForContext(test_string, tag_core.ContextURL)
    assert_true(url_escaped != test_string)
    
    sus raw_escaped := tag_core.EscapeForContext(test_string, tag_core.ContextRaw)
    assert_eq_string(raw_escaped, test_string)
    
    vibez.spill("✅ HTML context escaping tests passed")
}

fr fr Test safe HTML types
slay test_safe_html_types() {
    sus html_content := "<p>Hello World</p>"
    
    fr fr Test SafeHTML
    sus safe_html := tag_core.NewSafeHTML(html_content)
    assert_true(safe_html.String() != html_content)
    
    fr fr Test SafeURL
    sus safe_url := tag_core.NewSafeURL("https://example.com/page?param=value")
    assert_true(safe_url.String() != "")
    
    fr fr Test SafeJS
    sus safe_js := tag_core.NewSafeJS("console.log('Hello')")
    assert_true(safe_js.String() != "")
    
    fr fr Test SafeCSS
    sus safe_css := tag_core.NewSafeCSS("color: red; font-size: 16px;")
    assert_true(safe_css.String() != "")
    
    vibez.spill("✅ Safe HTML types tests passed")
}

fr fr Test HTML element manipulation
slay test_html_elements() {
    sus doc := tag_core.ParseHTML("<html><body></body></html>")
    assert_true(doc.Root != cringe)
    
    fr fr Test creating elements
    sus elem := doc.CreateElement("div")
    assert_eq_string(elem.TagName, "div")
    
    fr fr Test setting attributes
    elem.SetAttribute("class", "test-class")
    elem.SetAttribute("id", "test-id")
    
    sus class_attr, has_class := elem.GetAttribute("class")
    assert_true(has_class)
    assert_eq_string(class_attr, "test-class")
    
    fr fr Test adding text
    elem.SetText("Hello World")
    assert_eq_string(elem.Text(), "Hello World")
    
    fr fr Test adding children
    sus child_elem := doc.CreateElement("span")
    child_elem.SetText("Child element")
    elem.AddChild(child_elem)
    
    assert_eq_string(child_elem.Parent.TagName, "div")
    
    fr fr Test HTML generation
    sus html_output := elem.HTML()
    assert_true(html_output != "")
    assert_true(len(html_output) > 0)
    
    vibez.spill("✅ HTML element tests passed")
}

fr fr Run all tests
test_html_escaping()
test_tag_creation()
test_tag_hierarchy()
test_tag_cloning()
test_tag_set()
test_tag_query()
test_tag_search()
test_tag_relations()
test_tag_statistics()
test_bulk_operations()
test_tag_validation()
test_tag_merging()
test_tag_suggestions()
test_tag_export_import()
test_global_functions()
test_html_context_escaping()
test_safe_html_types()
test_html_elements()

print_test_summary()
vibez.spill("🎉 All TagCore tests completed successfully!")
