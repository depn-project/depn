mod vpn;

fn main() {
    let wireguard = vpn::WireGuard::new();

    let vpn_manager = vpn::Manager::new().protocol(&wireguard);

    let protocols = vpn_manager.available_protocols();

    vpn::print_protocols_table(protocols);
}
