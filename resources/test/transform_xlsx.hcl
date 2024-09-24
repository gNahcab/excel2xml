// which datamodel
// which xlsx files
// which sheets of which xlsx files
// separator
shortcode = 0828
resources_folder = "testdata/"
separator = "$£P"
datamodel = cmd.find
xlsx "CSVDocument.xlsx" {
  sheet "1" {
    resource = "CSVDocument"
    assignments {
      id = "label"
      rest = cmd.find
    }
  }
}
