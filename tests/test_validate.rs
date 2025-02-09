use crate::workdir::Workdir;

#[test]
fn validate_good_csv() {
    let wrk = Workdir::new("validate").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn validate_good_tab() {
    let wrk = Workdir::new("validate_good_tab").flexible(true);
    let tabfile = wrk.load_test_file("boston311-100.tab");
    let mut cmd = wrk.command("validate");
    cmd.arg(tabfile);

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"Valid: 29 Columns: ("case_enquiry_id", "open_dt", "target_dt", "closed_dt", "ontime", "case_status", "closure_reason", "case_title", "subject", "reason", "type", "queue", "department", "submittedphoto", "closedphoto", "location", "fire_district", "pwd_district", "city_council_district", "police_district", "neighborhood", "neighborhood_services_district", "ward", "precinct", "location_street_name", "location_zipcode", "latitude", "longitude", "source"); Records: 100; Delimiter: TAB"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_bad_tsv() {
    let wrk = Workdir::new("validate_bad_tsv").flexible(true);
    let tabfile = wrk.load_test_file("boston311-100-bad.tsv");
    let mut cmd = wrk.command("validate");
    cmd.arg(tabfile);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_good_csv_msg() {
    let wrk = Workdir::new("validate_good_csv_msg").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "real age (earth years)"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"Valid: 3 Columns: ("title", "name", "real age (earth years)"); Records: 3; Delimiter: ,"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_empty_csv_msg() {
    let wrk = Workdir::new("validate_empty_csv_msg").flexible(true);
    wrk.create(
        "data.csv",
        vec![svec!["title", "name", "real age (earth years)"]],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"Valid: 3 Columns: ("title", "name", "real age (earth years)"); Records: 0; Delimiter: ,"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_good_csv_pretty_json() {
    let wrk = Workdir::new("validate_good_csv_pretty_json").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "real age (earth years)"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--pretty-json").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{
  "delimiter_char": ",",
  "header_row": true,
  "quote_char": "\"",
  "num_records": 3,
  "num_fields": 3,
  "fields": [
    "title",
    "name",
    "real age (earth years)"
  ]
}"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_good_csv_json() {
    let wrk = Workdir::new("validate_good_csv_json").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--json").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"delimiter_char":",","header_row":true,"quote_char":"\"","num_records":3,"num_fields":3,"fields":["title","name","age"]}"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_bad_csv() {
    let wrk = Workdir::new("validate").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 2 (line: 3, byte: 36): found record with 2 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_first_record() {
    let wrk = Workdir::new("validate_bad_csv_first_record").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers",],
            svec!["Doctor", "Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 1 (line: 2, byte: 15): found record with 2 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_last_record() {
    let wrk = Workdir::new("validate_bad_csv_last_record").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Doctor", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14", "extra field"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 3 (line: 4, byte: 54): found record with 4 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_prettyjson() {
    let wrk = Workdir::new("validate_bad_csv_prettyjson").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--pretty-json").arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"{
  "errors": [
    {
      "title": "Validation error",
      "detail": "CSV error: record 2 (line: 3, byte: 36): found record with 2 fields, but the previous record has 3 fields",
      "meta": {
        "last_valid_record": "1"
      }
    }
  ]
}
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

fn adur_errors() -> &'static str {
    r#"row_number	field	error
1	ExtractDate	null is not of type "string"
1	OrganisationLabel	null is not of type "string"
3	CoordinateReferenceSystem	"OSGB3" does not match "(WGS84|OSGB36)"
3	Category	"Mens" does not match "(Female|Male|Female and Male|Unisex|Male urinal|Children only|None)"
"#
}

// invalid records with index from original csv
// row 1: missing values for ExtractDate and OrganisationLabel
// row 3: wrong value for CoordinateReferenceSystem and Category
// note: removed unnecessary quotes for string column "OpeningHours"
fn adur_invalids() -> &'static str {
    r#"ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00 ,ADC,surveyor_1@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,
2014-07-07 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,OSGB3,518915,103795,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Mens,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00,ADC,surveyor_3@adur-worthing.gov.uk,01903 221471,,60007428,,,,
"#
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema() {
    let wrk = Workdir::new("validate").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    wrk.output(&mut cmd);

    // check invalid file output
    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));
    assert_eq!(adur_invalids().to_string(), invalid_output);

    // check validation error output

    let validation_error_output: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));
    assert_eq!(adur_errors(), validation_error_output);
    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema_valid_output() {
    let wrk = Workdir::new("validate_valid_output").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets-valid.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv")
        .arg("schema.json")
        .args(["--valid-output", "-"]);

    let out = wrk.output_stderr(&mut cmd);
    let expected = "13\n";
    assert_eq!(out, expected);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ExtractDate", "OrganisationURI", "OrganisationLabel", "ServiceTypeURI", "ServiceTypeLabel", "LocationText", "CoordinateReferenceSystem", "GeoX", "GeoY", "GeoPointLicensingURL", "Category", "AccessibleCategory", "RADARKeyNeeded", "BabyChange", "FamilyToilet", "ChangingPlace", "AutomaticPublicConvenience", "FullTimeStaffing", "PartOfCommunityScheme", "CommunitySchemeName", "ChargeAmount", "InfoURL", "OpeningHours", "ManagedBy", "ReportEmail", "ReportTel", "Notes", "UPRN", "Postcode", "StreetAddress", "GeoAreaURI", "GeoAreaLabel"], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING", "OSGB36", "518225", "104730", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 15:00 W = 09:00 - 15:00", "ADC", "surveyor_2@adur-worthing.gov.uk", "01903 221471", "", "60002210", "", "PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES YEW TREE CLOSE LANCING", "OSGB36", "518222", "104168", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_4@adur-worthing.gov.uk", "01903 221471", "", "60008859", "", "PUBLIC CONVENIENCES YEW TREE CLOSE LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA", "OSGB36", "521299", "104515", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_5@adur-worthing.gov.uk", "01903 221471", "", "60009402", "", "PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA", "OSGB36", "521048", "104977", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 08:00 - 21:00 W = 08:00 - 17:00", "ADC", "surveyor_6@adur-worthing.gov.uk", "01903 221471", "", "60009666", "", "PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA", "OSGB36", "523294", "104588", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_7@adur-worthing.gov.uk", "01903 221471", "", "60011970", "", "PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA", "OSGB36", "521515", "105083", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_8@adur-worthing.gov.uk", "01903 221471", "", "60014163", "", "PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA", "OSGB36", "521440", "105725", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "", "ADC", "surveyor_9@adur-worthing.gov.uk", "01903 221471", "Grounds staff only not public", "60014340", "", "PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "OSGB36", "522118", "105939", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_10@adur-worthing.gov.uk", "01903 221471", "", "60017866", "", "PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK", "OSGB36", "524401", "105405", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 08:00 - 21:00 W = 08:00 - 17:00", "ADC", "surveyor_11@adur-worthing.gov.uk", "01903 221471", "", "60026354", "", "PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING", "OSGB36", "520354", "104246", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "", "surveyor_12@adur-worthing.gov.uk", "01903 221471", "", "60028994", "", "WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "524375", "104753", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_13@adur-worthing.gov.uk", "01903 221471", "", "60029181", "", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "522007", "106062", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "", "ADC", "surveyor_14@adur-worthing.gov.uk", "01903 221471", "Grounds staff only not public", "60032527", "", "PUBLIC CONVENIENCE NORTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "522083", "105168", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "09.00 - 17.00", "ADC", "surveyor_15@adur-worthing.gov.uk", "01903 221471", "", "60034215", "", "PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA", "", ""]    
    ];
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_with_schema_noheader() {
    let wrk = Workdir::new("validate_with_schema_noheader").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets-valid.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv")
        .arg("schema.json")
        .arg("--no-headers")
        .args(["--valid-output", "-"]);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "Cannot validate CSV without headers against a JSON Schema.\n".to_string();
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema_url() {
    let wrk = Workdir::new("validate").flexible(true);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("https://raw.githubusercontent.com/dathere/qsv/master/resources/test/public-toilets-schema.json");

    wrk.output(&mut cmd);

    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));
    assert_eq!(adur_invalids().to_string(), invalid_output);

    // check validation error output
    let validation_error_output: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));
    assert_eq!(adur_errors(), validation_error_output);
    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_column() {
    let wrk = Workdir::new("validate_dynenum_with_column").flexible(true);

    // Create lookup file first
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name", "category"],
            svec!["A1", "Apple", "fruit"],
            svec!["B2", "Banana", "fruit"],
            svec!["C3", "Carrot", "vegetable"],
        ],
    );

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "product", "type"],
            svec!["1", "Apple", "fruit"],
            svec!["2", "Banana", "fruit"],
            svec!["3", "Orange", "fruit"], // Invalid - not in lookup
            svec!["4", "Grape", "fruit"],  // Invalid - not in lookup
        ],
    );

    // Create schema using dynamicEnum with column specification
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "product": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|name"
                },
                "type": { "type": "string" }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));

    let expected_errors = "row_number\tfield\terror\n3\tproduct\t\"Orange\" is not a valid \
                           dynamicEnum value\n4\tproduct\t\"Grape\" is not a valid dynamicEnum \
                           value\n";
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["1", "Apple", "fruit"], svec!["2", "Banana", "fruit"]];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["3", "Orange", "fruit"], svec!["4", "Grape", "fruit"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_column_index() {
    let wrk = Workdir::new("validate_dynenum_with_column_index").flexible(true);

    // Create a sample CSV file with multiple columns
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name", "category"],
            svec!["A1", "Apple", "fruit"],
            svec!["B2", "Banana", "fruit"],
            svec!["C3", "Carrot", "vegetable"],
        ],
    );

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "category", "code"],
            svec!["1", "fruit", "A1"],
            svec!["2", "vegetable", "D4"], // Invalid - code not in lookup
            svec!["3", "fruit", "B2"],
            svec!["4", "fruit", "X9"], // Invalid - code not in lookup
        ],
    );

    // Create schema using dynamicEnum with column index
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "category": { "type": "string" },
                "code": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|0"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = "row_number\tfield\terror\n2\tcode\t\"D4\" is not a valid dynamicEnum \
                           value\n4\tcode\t\"X9\" is not a valid dynamicEnum value\n";
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["1", "fruit", "A1"], svec!["3", "fruit", "B2"]];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["2", "vegetable", "D4"], svec!["4", "fruit", "X9"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_invalid_column() {
    let wrk = Workdir::new("validate_dynenum_with_invalid_column").flexible(true);

    // Create lookup file first
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name"],
            svec!["A1", "Apple"],
            svec!["B2", "Banana"],
        ],
    );

    // Create test data
    wrk.create("data.csv", vec![svec!["id", "name"], svec!["1", "Apple"]]);

    // Create schema using dynamicEnum with non-existent column
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|nonexistent_column"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    // Check error output
    let got = wrk.output_stderr(&mut cmd);
    #[cfg(feature = "lite")]
    assert_eq!(got, "1 out of 1 records invalid.\n");
    #[cfg(not(feature = "lite"))]
    assert!(got.ends_with(
        "Cannot compile JSONschema. error: Column 'nonexistent_column' not found in lookup \
         table\nTry running `qsv validate schema schema.json` to check the JSON Schema file.\n"
    ));

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_remote_csv() {
    let wrk = Workdir::new("validate_dynenum_with_remote_csv").flexible(true);

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "fruit"],
            svec!["1", "banana"],
            svec!["2", "mango"], // Invalid - not in fruits.csv
            svec!["3", "apple"],
            svec!["4", "dragonfruit"], // Invalid - not in fruits.csv
        ],
    );

    // Create schema using dynamicEnum with remote CSV
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "fruit": { 
                    "type": "string",
                    "dynamicEnum": "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = "row_number\tfield\terror\n2\tfruit\t\"mango\" is not a valid \
                           dynamicEnum value\n4\tfruit\t\"dragonfruit\" is not a valid \
                           dynamicEnum value\n";
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["1", "banana"], svec!["3", "apple"]];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["2", "mango"], svec!["4", "dragonfruit"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[cfg(feature = "lite")]
#[test]
fn validate_lite_dynenum_combinations() {
    let wrk = Workdir::new("validate_lite_dynenum_combinations").flexible(true);

    // Create lookup file first
    wrk.create(
        "lookup.csv",
        vec![
            svec!["id", "name", "category"],
            svec!["1", "Apple", "fruit"],
            svec!["2", "Banana", "fruit"],
            svec!["3", "Carrot", "vegetable"],
        ],
    );

    // Test cases with different dynamicEnum URI patterns
    let test_cases = vec![
        // Simple file path
        (
            "lookup.csv",
            vec![
                svec!["id", "product"],
                svec!["1", "Apple"],   // invalid
                svec!["2", "Orange"],  // invalid
            ],
            2,
        ),
        // File path with column name
        (
            "lookup.csv|name",
            vec![
                svec!["id", "product"],
                svec!["1", "Apple"],   // valid
                svec!["2", "Orange"],  // invalid
            ],
            1,
        ),
        // File path with column index (2nd col - 0-based index)
        (
            "lookup.csv|1",
            vec![
                svec!["id", "product"],
                svec!["1", "Apple"],   // valid
                svec!["2", "Orange"],  // invalid
            ],
            1,
        ),
        // HTTP URL
        (
            "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv",
            vec![
                svec!["id", "fruit"],
                svec!["1", "banana"],  // valid
                svec!["2", "mango"],   // invalid
            ],
            1,
        ),
        // HTTP URL with column
        (
            "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv|0",
            vec![
                svec!["id", "fruit"],
                svec!["1", "banana"],  // valid
                svec!["2", "mango"],   // invalid
            ],
            1,
        ),
        // HTTP URL with column by name
        (
            "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv|fruit",
            vec![
                svec!["id", "fruit"],
                svec!["1", "banana"],  // valid
                svec!["2", "mango"],   // invalid
                svec!["3", "strawberry"], // valid
            ],
            1,
        ),
    ];

    for (uri, data, expected_invalid_count) in test_cases {
        // Create schema using dynamicEnum
        let schema = format!(
            r#"{{
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {{
                    "id": {{ "type": "string" }},
                    "product": {{ 
                        "type": "string",
                        "dynamicEnum": "{}"
                    }}
                }}
            }}"#,
            uri
        );
        wrk.create_from_string("schema.json", &schema);

        // Create test data
        wrk.create("data.csv", data);

        // Run validate command
        let mut cmd = wrk.command("validate");
        cmd.arg("data.csv").arg("schema.json");
        wrk.output(&mut cmd);

        // Check validation errors count
        let validation_errors = wrk
            .read_to_string("data.csv.validation-errors.tsv")
            .unwrap();
        let error_count = validation_errors.lines().count() - 1; // subtract header row
        assert_eq!(
            error_count, expected_invalid_count,
            "Failed for URI: {}",
            uri
        );
    }
}
