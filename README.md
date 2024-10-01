# excel2xml
transform excel-files to xml-files that can be uploaded to the DaSCH platform without touching the excel-files. We don't have to manipulate the excel files instead we write a hcl-file, where we describe what should be manipulated in the excel-file (only headers for now).

## parse-info-hcl
general:
- shortcode= (String) shortcode of project
- resources_folder= (String) path to resources folder
- separator= (String)
- datamodel= (String or command) path to datamodel or <cmd.find>

each excel-file: 
- xlsx: relative path
  - sheet: sheet number

each excel-sheet:
- resource: (String) name of resource (according to datamodel)
  - assignments

assignments: keys must exist in headers 

special case in assignments:
- rest= (String or command) can be a propname or <cmd.find>

## terminal commands
todo!
