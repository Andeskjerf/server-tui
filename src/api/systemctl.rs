pub fn get_units(state: &str) -> Option<Vec<String>> {
    let systemctl = systemctl::SystemCtl::default();

    let state_filter = if state.is_empty() { None } else { Some(state) };

    match systemctl.list_units(Some("service"), state_filter, None) {
        Ok(res) => Some(res),
        Err(_) => {
            println!("err: unable to determine running services");
            None
        }
    }
}
