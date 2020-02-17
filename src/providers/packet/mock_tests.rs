use mockito::{self, Matcher};
use providers::{packet, MetadataProvider};

#[test]
fn test_boot_checkin() {
    let data = packet::PacketData {
        id: String::new(),
        hostname: String::new(),
        iqn: String::new(),
        plan: String::new(),
        facility: String::new(),
        tags: vec![],
        ssh_keys: vec![],
        network: packet::PacketNetworkInfo {
            interfaces: vec![],
            addresses: vec![],
            bonding: packet::PacketBondingMode { mode: 0 },
        },
        error: None,
        phone_home_url: mockito::server_url(),
    };
    let provider = packet::PacketProvider { data };

    let mock = mockito::mock("POST", "/")
        .match_header(
            "content-type",
            Matcher::Regex("application/json".to_string()),
        )
        .match_body("")
        .with_status(200)
        .create();

    let r = provider.boot_checkin();
    mock.assert();
    r.unwrap();
}

#[test]
fn test_attributes() {
    let metadata = r#"{
        "id": "fake",
        "hostname": "test",
        "iqn": "fake",
        "plan": "baremetal_0",
        "facility": "ams1",
        "tags": [],
        "ssh_keys": [
          "fake"
        ],
        "network": {
          "bonding": {
            "mode": 5
          },
          "interfaces": [
            {
              "name": "eth0",
              "mac": "0c:c4:7a:b5:86:fc",
              "bond": "bond0"
            },
            {
              "name": "eth1",
              "mac": "0c:c4:7a:b5:86:fd",
              "bond": "bond0"
            }
          ],
          "addresses": [
            {
              "id": "fde74ec8-bc24-43ca-a852-875bd6e10bee",
              "address_family": 4,
              "netmask": "255.255.255.254",
              "public": true,
              "management": true,
              "address": "147.0.0.1",
              "gateway": "147.0.0.0"
            },
            {
              "id": "3ae01206-b03a-4353-b04d-94747d36457e",
              "address_family": 6,
              "netmask": "ffff:ffff:ffff:ffff:ffff:ffff:ffff:fffe",
              "public": true,
              "management": true,
              "address": "2604:1380::1",
              "gateway": "2604:1380::0"
            },
            {
              "id": "bec04697-31fd-4a99-b1a9-cd1c7b88c810",
              "address_family": 4,
              "netmask": "255.255.255.254",
              "public": false,
              "management": true,
              "address": "10.0.0.1",
              "gateway": "10.0.0.0"
            },
            {
              "id": "38475859-389f-48ac-a855-f3ebdb82c565",
              "address_family": 6,
              "netmask": "ffff:ffff:ffff:ffff:ffff:ffff:ffff:fffe",
              "public": false,
              "management": true,
              "address": "fd00::1",
              "gateway": "fd00::0"
            }
          ]
        },
        "phone_home_url": "http://localhost/phone-home"
      }"#;
    let _m = mockito::mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(metadata)
        .create();

    let provider = packet::PacketProvider::try_new().unwrap();
    let v = provider.attributes().unwrap();
    assert_eq!(v["PACKET_HOSTNAME"], "test");
    assert_eq!(v["PACKET_PHONE_HOME_URL"], "http://localhost/phone-home");
    assert_eq!(v["PACKET_IPV4_PUBLIC_0"], "147.0.0.1");
    assert_eq!(v["PACKET_IPV4_PUBLIC_GW_0"], "147.0.0.0");
    assert_eq!(v["PACKET_IPV4_PRIVATE_0"], "10.0.0.1");
    assert_eq!(v["PACKET_IPV4_PRIVATE_GW_0"], "10.0.0.0");
    assert_eq!(v["PACKET_IPV6_PUBLIC_0"], "2604:1380::1");
    assert_eq!(v["PACKET_IPV6_PUBLIC_GW_0"], "2604:1380::");
    assert_eq!(v["PACKET_IPV6_PRIVATE_0"], "fd00::1");
    assert_eq!(v["PACKET_IPV6_PRIVATE_GW_0"], "fd00::");
}
