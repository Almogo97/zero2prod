pub fn assert_status_code(expected: u16, response: reqwest::Response) {
    assert_eq!(expected, response.status().as_u16());
}
