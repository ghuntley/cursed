vibez.spill("JSON Tea Module Demo")

slay marshal_string(data tea) tea {
    damn "\"" + data + "\""
}

slay marshal_number(data tea) tea {
    damn data
}

slay marshal_boolean(data tea) tea {
    damn "true"
}

# Test basic marshaling
sus str_result tea = marshal_string("hello")
vibez.spill("String marshal: " + str_result)

sus num_result tea = marshal_number("42")
vibez.spill("Number marshal: " + num_result)

sus bool_result tea = marshal_boolean("based")
vibez.spill("Boolean marshal: " + bool_result)

vibez.spill("Demo complete!")
