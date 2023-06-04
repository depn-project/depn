mod vpn;

fn main() {
    let wireguard = vpn::WireGuard::new();
    let open_vpn = vpn::OpenVPN::new();

    let vpn_manager = vpn::Manager::new().protocol(&wireguard).protocol(&open_vpn);

    let protocols = vpn_manager.available_protocols();

    vpn::print_protocols_table(protocols);
}
