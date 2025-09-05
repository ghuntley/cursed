fr fr CONTACT MANAGEMENT SYSTEM - Complete CRM Application
fr fr Features: Contact storage, search, filtering, import/export, web interface, validation

yeet "database_enhanced"
yeet "webz"
yeet "json"
yeet "stringz"
yeet "timez"
yeet "fs"
yeet "vibez"

fr fr ===== APPLICATION CONFIGURATION =====

squad ContactConfig {
    sus database_url tea
    sus server_port drip
    sus web_root tea
    sus data_path tea
    sus backup_path tea
    sus max_contacts drip
}

sus contact_config ContactConfig = ContactConfig{
    database_url: "file://./contacts_data",
    server_port: 8088,
    web_root: "./contact_web",
    data_path: "./contacts_data",
    backup_path: "./contacts_backup",
    max_contacts: 10000
}

fr fr ===== DATA MODELS =====

squad Contact {
    sus id drip
    sus first_name tea
    sus last_name tea
    sus full_name tea
    sus email tea
    sus phone tea
    sus mobile tea
    sus company tea
    sus job_title tea
    sus address tea
    sus city tea
    sus state tea
    sus country tea
    sus postal_code tea
    sus website tea
    sus notes tea
    sus tags tea
    sus category tea
    sus is_favorite lit
    sus created_at tea
    sus updated_at tea
    sus last_contacted tea
}

squad ContactGroup {
    sus id drip
    sus name tea
    sus description tea
    sus contact_count drip
    sus created_at tea
}

squad ContactNote {
    sus id drip
    sus contact_id drip
    sus note tea
    sus note_type tea
    sus created_at tea
}

fr fr ===== DATABASE INITIALIZATION =====

slay initialize_contacts_database() database_enhanced.DatabaseConnection {
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(contact_config.database_url)
    
    ready (!conn.is_connected) {
        vibez.spill("FATAL: Could not connect to contacts database")
        sus empty database_enhanced.DatabaseConnection = database_enhanced.DatabaseConnection{}
        damn empty
    }
    
    fr fr Create all tables
    create_contacts_table(conn)
    create_contact_groups_table(conn)
    create_contact_notes_table(conn)
    create_contact_group_members_table(conn)
    
    fr fr Insert sample data
    insert_sample_contacts(conn)
    
    vibez.spill("Contacts database initialized successfully")
    damn conn
}

slay create_contacts_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "first_name": "TEXT NOT NULL",
        "last_name": "TEXT NOT NULL",
        "full_name": "TEXT NOT NULL",
        "email": "TEXT UNIQUE",
        "phone": "TEXT",
        "mobile": "TEXT",
        "company": "TEXT",
        "job_title": "TEXT",
        "address": "TEXT",
        "city": "TEXT",
        "state": "TEXT",
        "country": "TEXT",
        "postal_code": "TEXT",
        "website": "TEXT",
        "notes": "TEXT",
        "tags": "TEXT",
        "category": "TEXT DEFAULT 'General'",
        "is_favorite": "BOOLEAN DEFAULT 0",
        "created_at": "TEXT NOT NULL",
        "updated_at": "TEXT NOT NULL",
        "last_contacted": "TEXT"
    })
    
    ready (database_enhanced.create_table(conn, "contacts", schema)) {
        vibez.spill("Created contacts table")
    }
}

slay create_contact_groups_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "name": "TEXT NOT NULL UNIQUE",
        "description": "TEXT",
        "contact_count": "INTEGER DEFAULT 0",
        "created_at": "TEXT NOT NULL"
    })
    
    ready (database_enhanced.create_table(conn, "contact_groups", schema)) {
        vibez.spill("Created contact_groups table")
    }
}

slay create_contact_notes_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "contact_id": "INTEGER NOT NULL",
        "note": "TEXT NOT NULL",
        "note_type": "TEXT DEFAULT 'General'",
        "created_at": "TEXT NOT NULL"
    })
    
    ready (database_enhanced.create_table(conn, "contact_notes", schema)) {
        vibez.spill("Created contact_notes table")
    }
}

slay create_contact_group_members_table(conn database_enhanced.DatabaseConnection) {
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "group_id": "INTEGER NOT NULL",
        "contact_id": "INTEGER NOT NULL",
        "added_at": "TEXT NOT NULL"
    })
    
    ready (database_enhanced.create_table(conn, "contact_group_members", schema)) {
        vibez.spill("Created contact_group_members table")
    }
}

slay insert_sample_contacts(conn database_enhanced.DatabaseConnection) {
    sus now tea = timez.format_iso8601(timez.now_millis())
    
    sus sample_contacts []tea = [
        json.object_to_string({
            "first_name": "John",
            "last_name": "Doe",
            "full_name": "John Doe",
            "email": "john.doe@example.com",
            "phone": "+1-555-0101",
            "mobile": "+1-555-0102",
            "company": "Tech Solutions Inc",
            "job_title": "Software Engineer",
            "address": "123 Main Street",
            "city": "San Francisco",
            "state": "CA",
            "country": "USA",
            "postal_code": "94105",
            "website": "https://johndoe.dev",
            "notes": "Met at tech conference 2024",
            "tags": "developer,colleague,tech",
            "category": "Business",
            "is_favorite": "true",
            "created_at": now,
            "updated_at": now,
            "last_contacted": now
        }),
        json.object_to_string({
            "first_name": "Jane",
            "last_name": "Smith",
            "full_name": "Jane Smith",
            "email": "jane.smith@design.com",
            "phone": "+1-555-0201",
            "mobile": "+1-555-0202",
            "company": "Creative Designs LLC",
            "job_title": "UX Designer",
            "address": "456 Oak Avenue",
            "city": "New York",
            "state": "NY",
            "country": "USA",
            "postal_code": "10001",
            "website": "https://janesmith.design",
            "notes": "Excellent designer, very creative",
            "tags": "designer,creative,freelancer",
            "category": "Business",
            "is_favorite": "false",
            "created_at": now,
            "updated_at": now,
            "last_contacted": ""
        }),
        json.object_to_string({
            "first_name": "Bob",
            "last_name": "Johnson",
            "full_name": "Bob Johnson",
            "email": "bob@personal.com",
            "phone": "+1-555-0301",
            "mobile": "+1-555-0302",
            "company": "",
            "job_title": "",
            "address": "789 Pine Street",
            "city": "Austin",
            "state": "TX",
            "country": "USA",
            "postal_code": "78701",
            "website": "",
            "notes": "Family friend from college",
            "tags": "friend,personal,college",
            "category": "Personal",
            "is_favorite": "true",
            "created_at": now,
            "updated_at": now,
            "last_contacted": now
        })
    ]
    
    sus i drip = 0
    bestie (i < sample_contacts.length) {
        database_enhanced.insert_record(conn, "contacts", sample_contacts[i])
        i = i + 1
    }
    
    fr fr Create default groups
    sus default_groups []tea = [
        json.object_to_string({
            "name": "Business Contacts",
            "description": "Professional and business relationships",
            "contact_count": "0",
            "created_at": now
        }),
        json.object_to_string({
            "name": "Personal Friends",
            "description": "Personal friends and family",
            "contact_count": "0",
            "created_at": now
        }),
        json.object_to_string({
            "name": "Vendors & Suppliers",
            "description": "Business vendors and service providers",
            "contact_count": "0",
            "created_at": now
        })
    ]
    
    i = 0
    bestie (i < default_groups.length) {
        database_enhanced.insert_record(conn, "contact_groups", default_groups[i])
        i = i + 1
    }
    
    vibez.spill("Inserted sample contact data")
}

fr fr ===== CONTACT OPERATIONS =====

slay create_contact(conn database_enhanced.DatabaseConnection, contact_data map[tea]tea) drip {
    sus now tea = timez.format_iso8601(timez.now_millis())
    
    fr fr Build full name
    sus full_name tea = stringz.trim(contact_data["first_name"] + " " + contact_data["last_name"])
    
    fr fr Validate required fields
    ready (contact_data["first_name"] == "" || contact_data["last_name"] == "") {
        vibez.spill("ERROR: First name and last name are required")
        damn 0
    }
    
    fr fr Validate email format if provided
    ready (contact_data["email"] != "" && !is_valid_email(contact_data["email"])) {
        vibez.spill("ERROR: Invalid email format")
        damn 0
    }
    
    sus final_data tea = json.object_to_string({
        "first_name": contact_data["first_name"],
        "last_name": contact_data["last_name"],
        "full_name": full_name,
        "email": contact_data["email"],
        "phone": contact_data["phone"],
        "mobile": contact_data["mobile"],
        "company": contact_data["company"],
        "job_title": contact_data["job_title"],
        "address": contact_data["address"],
        "city": contact_data["city"],
        "state": contact_data["state"],
        "country": contact_data["country"],
        "postal_code": contact_data["postal_code"],
        "website": contact_data["website"],
        "notes": contact_data["notes"],
        "tags": contact_data["tags"],
        "category": contact_data["category"],
        "is_favorite": contact_data["is_favorite"],
        "created_at": now,
        "updated_at": now,
        "last_contacted": ""
    })
    
    ready (database_enhanced.insert_record(conn, "contacts", final_data)) {
        vibez.spill("Created contact: " + full_name)
        damn mathz.random_int(10000)
    }
    
    damn 0
}

slay get_all_contacts(conn database_enhanced.DatabaseConnection) []tea {
    sus conditions tea = "{}"
    sus contacts []tea = database_enhanced.find_records(conn, "contacts", conditions)
    
    vibez.spill("Retrieved " + stringz.from_int(contacts.length) + " contacts")
    damn contacts
}

slay search_contacts(conn database_enhanced.DatabaseConnection, search_term tea) []tea {
    sus all_contacts []tea = get_all_contacts(conn)
    sus matching_contacts []tea = []
    sus match_count drip = 0
    
    sus search_lower tea = stringz.to_lower(search_term)
    
    sus i drip = 0
    bestie (i < all_contacts.length) {
        sus contact_data map[tea]tea = json.parse_object(all_contacts[i])
        
        ready (contact_matches_search(contact_data, search_lower)) {
            matching_contacts[match_count] = all_contacts[i]
            match_count = match_count + 1
        }
        
        i = i + 1
    }
    
    vibez.spill("Found " + stringz.from_int(match_count) + " contacts matching: " + search_term)
    damn matching_contacts
}

slay contact_matches_search(contact_data map[tea]tea, search_term tea) lit {
    sus fields []tea = ["full_name", "email", "phone", "mobile", "company", "job_title", "city", "tags"]
    
    sus i drip = 0
    bestie (i < fields.length) {
        sus field_value tea = stringz.to_lower(contact_data[fields[i]])
        ready (stringz.contains(field_value, search_term)) {
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

slay get_contacts_by_category(conn database_enhanced.DatabaseConnection, category tea) []tea {
    sus conditions tea = json.object_to_string({
        "category": category
    })
    
    sus contacts []tea = database_enhanced.find_records(conn, "contacts", conditions)
    vibez.spill("Retrieved " + stringz.from_int(contacts.length) + " contacts in category: " + category)
    damn contacts
}

slay get_favorite_contacts(conn database_enhanced.DatabaseConnection) []tea {
    sus conditions tea = json.object_to_string({
        "is_favorite": "true"
    })
    
    sus contacts []tea = database_enhanced.find_records(conn, "contacts", conditions)
    vibez.spill("Retrieved " + stringz.from_int(contacts.length) + " favorite contacts")
    damn contacts
}

slay update_contact(conn database_enhanced.DatabaseConnection, id drip, updates map[tea]tea) lit {
    fr fr Add updated timestamp
    updates["updated_at"] = timez.format_iso8601(timez.now_millis())
    
    fr fr Update full name if first/last name changed
    ready (updates["first_name"] != "" || updates["last_name"] != "") {
        sus first_name tea = updates["first_name"]
        sus last_name tea = updates["last_name"]
        
        fr fr If only one name is updated, get the other from database
        ready (first_name == "" || last_name == "") {
            sus existing_contact tea = get_contact_by_id(conn, id)
            ready (existing_contact != "") {
                sus existing_data map[tea]tea = json.parse_object(existing_contact)
                ready (first_name == "") {
                    first_name = existing_data["first_name"]
                }
                ready (last_name == "") {
                    last_name = existing_data["last_name"]
                }
            }
        }
        
        updates["full_name"] = stringz.trim(first_name + " " + last_name)
    }
    
    sus final_updates tea = json.object_to_string(updates)
    
    ready (database_enhanced.update_record(conn, "contacts", id, final_updates)) {
        vibez.spill("Updated contact ID: " + stringz.from_int(id))
        damn based
    }
    
    damn cringe
}

slay delete_contact(conn database_enhanced.DatabaseConnection, id drip) lit {
    ready (database_enhanced.delete_record(conn, "contacts", id)) {
        vibez.spill("Deleted contact ID: " + stringz.from_int(id))
        damn based
    }
    
    damn cringe
}

slay get_contact_by_id(conn database_enhanced.DatabaseConnection, id drip) tea {
    sus conditions tea = json.object_to_string({
        "id": stringz.from_int(id)
    })
    
    sus contacts []tea = database_enhanced.find_records(conn, "contacts", conditions)
    ready (contacts.length > 0) {
        damn contacts[0]
    }
    
    damn ""
}

fr fr ===== IMPORT/EXPORT FUNCTIONALITY =====

slay export_contacts_to_csv(conn database_enhanced.DatabaseConnection, file_path tea) lit {
    sus contacts []tea = get_all_contacts(conn)
    
    fr fr CSV header
    sus csv_content tea = "ID,First Name,Last Name,Email,Phone,Mobile,Company,Job Title,Address,City,State,Country,Postal Code,Website,Notes,Tags,Category,Favorite,Created At\n"
    
    sus i drip = 0
    bestie (i < contacts.length) {
        sus contact_data map[tea]tea = json.parse_object(contacts[i])
        
        sus csv_row tea = format_csv_row([
            contact_data["id"],
            contact_data["first_name"],
            contact_data["last_name"],
            contact_data["email"],
            contact_data["phone"],
            contact_data["mobile"],
            contact_data["company"],
            contact_data["job_title"],
            contact_data["address"],
            contact_data["city"],
            contact_data["state"],
            contact_data["country"],
            contact_data["postal_code"],
            contact_data["website"],
            contact_data["notes"],
            contact_data["tags"],
            contact_data["category"],
            contact_data["is_favorite"],
            contact_data["created_at"]
        ])
        
        csv_content = csv_content + csv_row + "\n"
        i = i + 1
    }
    
    ready (fs.write_file(file_path, csv_content)) {
        vibez.spill("Exported " + stringz.from_int(contacts.length) + " contacts to CSV: " + file_path)
        damn based
    }
    
    damn cringe
}

slay export_contacts_to_json(conn database_enhanced.DatabaseConnection, file_path tea) lit {
    sus contacts []tea = get_all_contacts(conn)
    
    sus export_data tea = json.object_to_string({
        "export_date": timez.format_iso8601(timez.now_millis()),
        "contact_count": stringz.from_int(contacts.length),
        "contacts": json.array_to_string(contacts)
    })
    
    ready (fs.write_file(file_path, export_data)) {
        vibez.spill("Exported " + stringz.from_int(contacts.length) + " contacts to JSON: " + file_path)
        damn based
    }
    
    damn cringe
}

slay import_contacts_from_csv(conn database_enhanced.DatabaseConnection, file_path tea) drip {
    ready (!fs.file_exists(file_path)) {
        vibez.spill("ERROR: CSV file not found: " + file_path)
        damn 0
    }
    
    sus csv_content tea = fs.read_file(file_path)
    sus lines []tea = stringz.split(csv_content, "\n")
    
    ready (lines.length <= 1) {
        vibez.spill("ERROR: CSV file is empty or has no data")
        damn 0
    }
    
    sus imported_count drip = 0
    sus i drip = 1 fr fr Skip header row
    
    bestie (i < lines.length) {
        sus line tea = stringz.trim(lines[i])
        ready (line != "") {
            sus contact_data map[tea]tea = parse_csv_contact_row(line)
            ready (contact_data["first_name"] != "") {
                sus id drip = create_contact(conn, contact_data)
                ready (id > 0) {
                    imported_count = imported_count + 1
                }
            }
        }
        i = i + 1
    }
    
    vibez.spill("Imported " + stringz.from_int(imported_count) + " contacts from CSV")
    damn imported_count
}

slay import_contacts_from_json(conn database_enhanced.DatabaseConnection, file_path tea) drip {
    ready (!fs.file_exists(file_path)) {
        vibez.spill("ERROR: JSON file not found: " + file_path)
        damn 0
    }
    
    sus json_content tea = fs.read_file(file_path)
    sus import_data map[tea]tea = json.parse_object(json_content)
    
    ready (import_data["contacts"] == "") {
        vibez.spill("ERROR: Invalid JSON import format")
        damn 0
    }
    
    sus contacts []tea = json.parse_array(import_data["contacts"])
    sus imported_count drip = 0
    
    sus i drip = 0
    bestie (i < contacts.length) {
        sus contact_data map[tea]tea = json.parse_object(contacts[i])
        ready (contact_data["first_name"] != "") {
            fr fr Remove ID to create new contact
            contact_data["id"] = ""
            sus id drip = create_contact(conn, contact_data)
            ready (id > 0) {
                imported_count = imported_count + 1
            }
        }
        i = i + 1
    }
    
    vibez.spill("Imported " + stringz.from_int(imported_count) + " contacts from JSON")
    damn imported_count
}

fr fr ===== WEB API HANDLERS =====

slay handle_contacts_api_request(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"}
    
    ready (request.method == "GET" && request.path == "/api/contacts") {
        damn handle_get_contacts(conn, request)
    } otherwise ready (request.method == "POST" && request.path == "/api/contacts") {
        damn handle_create_contact_api(conn, request)
    } otherwise ready (request.method == "GET" && stringz.starts_with(request.path, "/api/contacts/")) {
        damn handle_get_contact(conn, request)
    } otherwise ready (request.method == "PUT" && stringz.starts_with(request.path, "/api/contacts/")) {
        damn handle_update_contact_api(conn, request)
    } otherwise ready (request.method == "DELETE" && stringz.starts_with(request.path, "/api/contacts/")) {
        damn handle_delete_contact_api(conn, request)
    } otherwise ready (request.method == "GET" && request.path == "/api/contacts/search") {
        damn handle_search_contacts(conn, request)
    } otherwise ready (request.method == "GET" && request.path == "/api/contacts/export/csv") {
        damn handle_export_csv(conn, request)
    } otherwise ready (request.method == "GET" && request.path == "/api/contacts/export/json") {
        damn handle_export_json(conn, request)
    } otherwise ready (request.method == "POST" && request.path == "/api/contacts/import") {
        damn handle_import_contacts(conn, request)
    } otherwise {
        response.status_code = 404
        response.body = json.object_to_string({"error": "API endpoint not found"})
        damn response
    }
}

slay handle_get_contacts(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    response.status_code = 200
    
    fr fr Check for filters
    sus category tea = webz.get_query_param(request, "category")
    sus favorites tea = webz.get_query_param(request, "favorites")
    
    sus contacts []tea = []
    
    ready (category != "") {
        contacts = get_contacts_by_category(conn, category)
    } otherwise ready (favorites == "true") {
        contacts = get_favorite_contacts(conn)
    } otherwise {
        contacts = get_all_contacts(conn)
    }
    
    response.body = json.object_to_string({
        "contacts": json.array_to_string(contacts),
        "count": stringz.from_int(contacts.length)
    })
    
    damn response
}

slay handle_create_contact_api(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    sus contact_data map[tea]tea = json.parse_object(request.body)
    
    sus id drip = create_contact(conn, contact_data)
    
    ready (id > 0) {
        response.status_code = 201
        response.body = json.object_to_string({
            "id": stringz.from_int(id),
            "message": "Contact created successfully"
        })
    } otherwise {
        response.status_code = 400
        response.body = json.object_to_string({
            "error": "Failed to create contact. Check required fields."
        })
    }
    
    damn response
}

slay handle_search_contacts(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "application/json"}
    
    sus search_term tea = webz.get_query_param(request, "q")
    
    ready (search_term == "") {
        response.status_code = 400
        response.body = json.object_to_string({
            "error": "Search term is required"
        })
        damn response
    }
    
    sus contacts []tea = search_contacts(conn, search_term)
    
    response.status_code = 200
    response.body = json.object_to_string({
        "contacts": json.array_to_string(contacts),
        "count": stringz.from_int(contacts.length),
        "search_term": search_term
    })
    
    damn response
}

slay handle_export_csv(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    
    sus timestamp tea = stringz.from_int(timez.now_millis())
    sus filename tea = "contacts_export_" + timestamp + ".csv"
    sus file_path tea = contact_config.data_path + "/" + filename
    
    ready (export_contacts_to_csv(conn, file_path)) {
        response.status_code = 200
        response.headers = {
            "Content-Type": "text/csv",
            "Content-Disposition": "attachment; filename=" + filename
        }
        response.body = fs.read_file(file_path)
    } otherwise {
        response.status_code = 500
        response.headers = {"Content-Type": "application/json"}
        response.body = json.object_to_string({
            "error": "Failed to export contacts"
        })
    }
    
    damn response
}

fr fr ===== WEB INTERFACE HANDLERS =====

slay handle_contacts_web_request(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    ready (request.path == "/" || request.path == "/index.html") {
        damn render_contacts_dashboard(conn)
    } otherwise ready (request.path == "/contacts") {
        damn render_contacts_list(conn, request)
    } otherwise ready (request.path == "/contact/new") {
        damn render_contact_form(empty_contact_data())
    } otherwise ready (stringz.starts_with(request.path, "/contact/")) {
        damn render_contact_details(conn, request)
    } otherwise ready (request.path == "/import") {
        damn render_import_page()
    } otherwise ready (request.path == "/export") {
        damn render_export_page(conn)
    } otherwise {
        sus response webz.HttpResponse = webz.HttpResponse{}
        response.status_code = 404
        response.body = render_404_page()
        response.headers = {"Content-Type": "text/html"}
        damn response
    }
}

slay render_contacts_dashboard(conn database_enhanced.DatabaseConnection) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "text/html"}
    response.status_code = 200
    
    sus all_contacts []tea = get_all_contacts(conn)
    sus favorites []tea = get_favorite_contacts(conn)
    sus business_contacts []tea = get_contacts_by_category(conn, "Business")
    sus personal_contacts []tea = get_contacts_by_category(conn, "Personal")
    
    sus stats_data map[tea]tea = {
        "total_contacts": stringz.from_int(all_contacts.length),
        "favorite_contacts": stringz.from_int(favorites.length),
        "business_contacts": stringz.from_int(business_contacts.length),
        "personal_contacts": stringz.from_int(personal_contacts.length)
    }
    
    sus html tea = generate_dashboard_html(stats_data, favorites)
    response.body = html
    
    damn response
}

slay render_contacts_list(conn database_enhanced.DatabaseConnection, request webz.HttpRequest) webz.HttpResponse {
    sus response webz.HttpResponse = webz.HttpResponse{}
    response.headers = {"Content-Type": "text/html"}
    response.status_code = 200
    
    fr fr Handle search and filters
    sus search_term tea = webz.get_query_param(request, "search")
    sus category tea = webz.get_query_param(request, "category")
    
    sus contacts []tea = []
    sus page_title tea = "All Contacts"
    
    ready (search_term != "") {
        contacts = search_contacts(conn, search_term)
        page_title = "Search Results for '" + search_term + "'"
    } otherwise ready (category != "") {
        contacts = get_contacts_by_category(conn, category)
        page_title = "Contacts in " + category
    } otherwise {
        contacts = get_all_contacts(conn)
    }
    
    sus html tea = generate_contacts_list_html(contacts, page_title, search_term, category)
    response.body = html
    
    damn response
}

fr fr ===== HTML GENERATION =====

slay generate_dashboard_html(stats_data map[tea]tea, recent_contacts []tea) tea {
    sus html tea = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Contact Manager - Dashboard</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 0; background: #f8f9fa; }
        .header { background: linear-gradient(135deg, #007bff, #0056b3); color: white; padding: 20px 0; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .container { max-width: 1200px; margin: 0 auto; padding: 0 20px; }
        .nav { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
        .nav h1 { margin: 0; font-size: 2rem; }
        .nav-links a { color: rgba(255,255,255,0.9); text-decoration: none; margin: 0 15px; padding: 8px 16px; border-radius: 4px; transition: all 0.3s; }
        .nav-links a:hover, .nav-links a.active { background: rgba(255,255,255,0.2); color: white; }
        .stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 30px 0; }
        .stat-card { background: white; padding: 25px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); text-align: center; }
        .stat-number { font-size: 2.5rem; font-weight: bold; color: #007bff; margin-bottom: 10px; }
        .stat-label { color: #666; font-size: 1rem; }
        .main-content { display: grid; grid-template-columns: 2fr 1fr; gap: 30px; margin-top: 30px; }
        .card { background: white; padding: 25px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); margin-bottom: 20px; }
        .card h3 { margin: 0 0 20px 0; color: #2c3e50; }
        .contact-item { display: flex; align-items: center; padding: 15px 0; border-bottom: 1px solid #eee; }
        .contact-item:last-child { border-bottom: none; }
        .contact-avatar { width: 40px; height: 40px; background: #007bff; color: white; border-radius: 50%; display: flex; align-items: center; justify-content: center; margin-right: 15px; font-weight: bold; }
        .contact-info { flex: 1; }
        .contact-name { font-weight: 600; color: #2c3e50; margin-bottom: 4px; }
        .contact-details { font-size: 0.9em; color: #666; }
        .btn { background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; text-decoration: none; display: inline-block; font-size: 0.9rem; transition: all 0.3s; }
        .btn:hover { background: #0056b3; transform: translateY(-1px); }
        .btn-secondary { background: #6c757d; }
        .btn-success { background: #28a745; }
        .btn-warning { background: #ffc107; color: #212529; }
        .quick-actions { display: flex; gap: 10px; flex-wrap: wrap; }
        .search-box { width: 100%; padding: 12px; border: 1px solid #ddd; border-radius: 4px; font-size: 1rem; margin-bottom: 20px; }
        @media (max-width: 768px) { 
            .main-content { grid-template-columns: 1fr; }
            .stats-grid { grid-template-columns: repeat(2, 1fr); }
            .quick-actions { flex-direction: column; }
        }
    </style>
</head>
<body>
    <div class="header">
        <div class="container">
            <div class="nav">
                <h1>📇 Contact Manager</h1>
                <div class="nav-links">
                    <a href="/" class="active">Dashboard</a>
                    <a href="/contacts">All Contacts</a>
                    <a href="/contact/new">Add Contact</a>
                    <a href="/import">Import</a>
                    <a href="/export">Export</a>
                </div>
            </div>
        </div>
    </div>
    
    <div class="container">
        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-number">` + stats_data["total_contacts"] + `</div>
                <div class="stat-label">Total Contacts</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">` + stats_data["favorite_contacts"] + `</div>
                <div class="stat-label">Favorites</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">` + stats_data["business_contacts"] + `</div>
                <div class="stat-label">Business</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">` + stats_data["personal_contacts"] + `</div>
                <div class="stat-label">Personal</div>
            </div>
        </div>
        
        <div class="main-content">
            <main>
                <div class="card">
                    <h3>Quick Actions</h3>
                    <div class="quick-actions">
                        <a href="/contact/new" class="btn">➕ Add New Contact</a>
                        <a href="/contacts" class="btn btn-secondary">👥 View All Contacts</a>
                        <a href="/contacts?favorites=true" class="btn btn-warning">⭐ View Favorites</a>
                        <a href="/import" class="btn btn-success">📥 Import Contacts</a>
                        <a href="/export" class="btn btn-success">📤 Export Contacts</a>
                    </div>
                </div>
                
                <div class="card">
                    <h3>Search Contacts</h3>
                    <form method="GET" action="/contacts">
                        <input type="text" name="search" placeholder="Search by name, email, company, or phone..." class="search-box">
                        <button type="submit" class="btn">🔍 Search</button>
                    </form>
                </div>
            </main>
            
            <aside>
                <div class="card">
                    <h3>Favorite Contacts</h3>
                    ` + generate_contact_list_items(recent_contacts) + `
                    <a href="/contacts?favorites=true" class="btn" style="width: 100%; margin-top: 15px;">View All Favorites</a>
                </div>
                
                <div class="card">
                    <h3>Categories</h3>
                    <div style="display: flex; flex-direction: column; gap: 10px;">
                        <a href="/contacts?category=Business" class="btn btn-secondary">💼 Business (` + stats_data["business_contacts"] + `)</a>
                        <a href="/contacts?category=Personal" class="btn btn-secondary">👤 Personal (` + stats_data["personal_contacts"] + `)</a>
                        <a href="/contacts?category=General" class="btn btn-secondary">📋 General</a>
                    </div>
                </div>
            </aside>
        </div>
    </div>
    
    <footer style="background: #2c3e50; color: white; text-align: center; padding: 20px 0; margin-top: 40px;">
        <div class="container">
            <p>&copy; 2024 Contact Manager. Built with ❤️ using CURSED programming language.</p>
        </div>
    </footer>
</body>
</html>`
    
    damn html
}

slay generate_contact_list_items(contacts []tea) tea {
    ready (contacts.length == 0) {
        damn `<div class="contact-item">No favorite contacts yet.</div>`
    }
    
    sus html tea = ""
    sus i drip = 0
    bestie (i < contacts.length && i < 5) { fr fr Show max 5 in sidebar
        sus contact_data map[tea]tea = json.parse_object(contacts[i])
        sus initials tea = get_contact_initials(contact_data["first_name"], contact_data["last_name"])
        
        html = html + `<div class="contact-item">
            <div class="contact-avatar">` + initials + `</div>
            <div class="contact-info">
                <div class="contact-name">` + contact_data["full_name"] + `</div>
                <div class="contact-details">` + contact_data["email"] + `</div>
                <div class="contact-details">` + contact_data["phone"] + `</div>
            </div>
        </div>`
        
        i = i + 1
    }
    
    damn html
}

fr fr ===== UTILITY FUNCTIONS =====

slay is_valid_email(email tea) lit {
    fr fr Simple email validation
    ready (email == "") {
        damn based
    }
    
    ready (!stringz.contains(email, "@")) {
        damn cringe
    }
    
    sus at_index drip = stringz.find(email, "@")
    sus domain_part tea = stringz.substring(email, at_index + 1, stringz.length(email))
    
    ready (!stringz.contains(domain_part, ".")) {
        damn cringe
    }
    
    damn based
}

slay format_csv_row(fields []tea) tea {
    sus i drip = 0
    sus csv_row tea = ""
    
    bestie (i < fields.length) {
        ready (i > 0) {
            csv_row = csv_row + ","
        }
        
        fr fr Escape quotes and wrap in quotes if necessary
        sus field tea = stringz.replace_all(fields[i], "\"", "\"\"")
        ready (stringz.contains(field, ",") || stringz.contains(field, "\"") || stringz.contains(field, "\n")) {
            field = "\"" + field + "\""
        }
        
        csv_row = csv_row + field
        i = i + 1
    }
    
    damn csv_row
}

slay parse_csv_contact_row(csv_row tea) map[tea]tea {
    sus fields []tea = stringz.split(csv_row, ",")
    sus contact_data map[tea]tea = {}
    
    ready (fields.length >= 7) {
        contact_data["first_name"] = clean_csv_field(fields[1])
        contact_data["last_name"] = clean_csv_field(fields[2])
        contact_data["email"] = clean_csv_field(fields[3])
        contact_data["phone"] = clean_csv_field(fields[4])
        contact_data["mobile"] = clean_csv_field(fields[5])
        contact_data["company"] = clean_csv_field(fields[6])
        contact_data["job_title"] = ""
        contact_data["address"] = ""
        contact_data["city"] = ""
        contact_data["state"] = ""
        contact_data["country"] = ""
        contact_data["postal_code"] = ""
        contact_data["website"] = ""
        contact_data["notes"] = ""
        contact_data["tags"] = ""
        contact_data["category"] = "General"
        contact_data["is_favorite"] = "false"
    }
    
    damn contact_data
}

slay clean_csv_field(field tea) tea {
    sus cleaned tea = stringz.trim(field)
    ready (stringz.starts_with(cleaned, "\"") && stringz.ends_with(cleaned, "\"")) {
        cleaned = stringz.substring(cleaned, 1, stringz.length(cleaned) - 1)
        cleaned = stringz.replace_all(cleaned, "\"\"", "\"")
    }
    damn cleaned
}

slay get_contact_initials(first_name tea, last_name tea) tea {
    sus first_initial tea = ""
    sus last_initial tea = ""
    
    ready (stringz.length(first_name) > 0) {
        first_initial = stringz.substring(first_name, 0, 1)
    }
    
    ready (stringz.length(last_name) > 0) {
        last_initial = stringz.substring(last_name, 0, 1)
    }
    
    damn stringz.to_upper(first_initial + last_initial)
}

slay empty_contact_data() map[tea]tea {
    sus empty_data map[tea]tea = {
        "first_name": "",
        "last_name": "",
        "email": "",
        "phone": "",
        "mobile": "",
        "company": "",
        "job_title": "",
        "address": "",
        "city": "",
        "state": "",
        "country": "",
        "postal_code": "",
        "website": "",
        "notes": "",
        "tags": "",
        "category": "General",
        "is_favorite": "false"
    }
    damn empty_data
}

fr fr ===== MAIN APPLICATION =====

slay main_character() {
    vibez.spill("Starting Contact Management System...")
    
    fr fr Initialize database
    sus db_conn database_enhanced.DatabaseConnection = initialize_contacts_database()
    ready (!db_conn.is_connected) {
        vibez.spill("FATAL: Could not initialize contacts database")
        damn
    }
    
    fr fr Create data directories
    create_data_directories()
    
    fr fr Setup web server
    sus server webz.Server = webz.create_server(contact_config.server_port)
    
    fr fr Register request handlers
    webz.handle_requests(server, slay(request webz.HttpRequest) webz.HttpResponse {
        ready (stringz.starts_with(request.path, "/api/")) {
            damn handle_contacts_api_request(db_conn, request)
        } otherwise {
            damn handle_contacts_web_request(db_conn, request)
        }
    })
    
    vibez.spill("Contact Management System started on port " + stringz.from_int(contact_config.server_port))
    vibez.spill("Dashboard: http://localhost:" + stringz.from_int(contact_config.server_port))
    vibez.spill("API endpoints:")
    vibez.spill("  GET /api/contacts - List all contacts")
    vibez.spill("  POST /api/contacts - Create new contact")
    vibez.spill("  GET /api/contacts/search?q=term - Search contacts")
    vibez.spill("  GET /api/contacts/export/csv - Export to CSV")
    vibez.spill("  GET /api/contacts/export/json - Export to JSON")
    vibez.spill("  POST /api/contacts/import - Import from file")
    
    fr fr Start server
    webz.start_server(server)
    
    fr fr Cleanup
    database_enhanced.close_connection(db_conn)
}

slay create_data_directories() {
    sus directories []tea = [
        contact_config.data_path,
        contact_config.backup_path,
        contact_config.web_root
    ]
    
    sus i drip = 0
    bestie (i < directories.length) {
        ready (!fs.directory_exists(directories[i])) {
            fs.create_directory(directories[i])
        }
        i = i + 1
    }
}

fr fr Start the Contact Management System
main()
