use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
pub struct Record {
    #[serde(rename = "First Name")]
    pub first_name: String,

    #[serde(rename = "Middle Name")]
    pub middle_name: String,

    #[serde(rename = "Last Name")]
    pub last_name: String,

    #[serde(rename = "Phonetic First Name")]
    pub phonetic_first_name: String,

    #[serde(rename = "Phonetic Middle Name")]
    pub phonetic_middle_name: String,

    #[serde(rename = "Phonetic Last Name")]
    pub phonetic_last_name: String,

    #[serde(rename = "Name Prefix")]
    pub name_prefix: String,

    #[serde(rename = "Name Suffix")]
    pub name_suffix: String,

    #[serde(rename = "Nickname")]
    pub nickname: String,

    #[serde(rename = "File As")]
    pub file_as: String,

    #[serde(rename = "Organization Name")]
    pub organization_name: String,

    #[serde(rename = "Organization Title")]
    pub organization_title: String,

    #[serde(rename = "Organization Department")]
    pub organization_department: String,

    #[serde(rename = "Birthday")]
    pub birthday: String,

    #[serde(rename = "Notes")]
    pub notes: String,

    #[serde(rename = "Photo")]
    pub photo: String,

    #[serde(rename = "Labels")]
    pub labels: String,

    #[serde(rename = "E-mail 1 - Label")]
    pub email_1_label: String,

    #[serde(rename = "E-mail 1 - Value")]
    pub email_1_value: String,

    #[serde(rename = "E-mail 2 - Label")]
    pub email_2_label: String,

    #[serde(rename = "E-mail 2 - Value")]
    pub email_2_value: String,

    #[serde(rename = "E-mail 3 - Label")]
    pub email_3_label: String,

    #[serde(rename = "E-mail 3 - Value")]
    pub email_3_value: String,

    #[serde(rename = "E-mail 4 - Label")]
    pub email_4_label: String,

    #[serde(rename = "E-mail 4 - Value")]
    pub email_4_value: String,

    #[serde(rename = "Phone 1 - Label")]
    pub phone_1_label: String,

    #[serde(rename = "Phone 1 - Value")]
    pub phone_1_value: String,

    #[serde(rename = "Phone 2 - Label")]
    pub phone_2_label: String,

    #[serde(rename = "Phone 2 - Value")]
    pub phone_2_value: String,

    #[serde(rename = "Phone 3 - Label")]
    pub phone_3_label: String,

    #[serde(rename = "Phone 3 - Value")]
    pub phone_3_value: String,

    #[serde(rename = "Phone 4 - Label")]
    pub phone_4_label: String,

    #[serde(rename = "Phone 4 - Value")]
    pub phone_4_value: String,

    #[serde(rename = "Address 1 - Label")]
    pub address_1_label: String,

    #[serde(rename = "Address 1 - Formatted")]
    pub address_1_formatted: String,

    #[serde(rename = "Address 1 - Street")]
    pub address_1_street: String,

    #[serde(rename = "Address 1 - City")]
    pub address_1_city: String,

    #[serde(rename = "Address 1 - PO Box")]
    pub address_1_po_box: String,

    #[serde(rename = "Address 1 - Region")]
    pub address_1_region: String,

    #[serde(rename = "Address 1 - Postal Code")]
    pub address_1_postal_code: String,

    #[serde(rename = "Address 1 - Country")]
    pub address_1_country: String,

    #[serde(rename = "Address 1 - Extended Address")]
    pub address_1_extended_address: String,

    #[serde(rename = "Address 2 - Label")]
    pub address_2_label: String,

    #[serde(rename = "Address 2 - Formatted")]
    pub address_2_formatted: String,

    #[serde(rename = "Address 2 - Street")]
    pub address_2_street: String,

    #[serde(rename = "Address 2 - City")]
    pub address_2_city: String,

    #[serde(rename = "Address 2 - PO Box")]
    pub address_2_po_box: String,

    #[serde(rename = "Address 2 - Region")]
    pub address_2_region: String,

    #[serde(rename = "Address 2 - Postal Code")]
    pub address_2_postal_code: String,

    #[serde(rename = "Address 2 - Country")]
    pub address_2_country: String,

    #[serde(rename = "Address 2 - Extended Address")]
    pub address_2_extended_address: String,

    #[serde(rename = "Relation 1 - Label")]
    pub relation_1_label: String,

    #[serde(rename = "Relation 1 - Value")]
    pub relation_1_value: String,

    #[serde(rename = "Website 1 - Label")]
    pub website_1_label: String,

    #[serde(rename = "Website 1 - Value")]
    pub website_1_value: String,

    #[serde(rename = "Event 1 - Label")]
    pub event_1_label: String,

    #[serde(rename = "Event 1 - Value")]
    pub event_1_value: String,

    #[serde(rename = "Custom Field 1 - Label")]
    pub custom_field_1_label: String,

    #[serde(rename = "Custom Field 1 - Value")]
    pub custom_field_1_value: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 42;
        assert_eq!(result, 42);
    }
}
