fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut conf = prost_build::Config::new();
        conf.out_dir("src/pb/prostpb/")
        .type_attribute(
            "delivery.Store",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .field_attribute(
            "delivery.Store.products",
            "#[serde(skip_serializing_if = \"Vec::is_empty\", default)]",
        )
        .type_attribute(
            "delivery.Product",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "delivery.Money",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile_protos(
            &["proto/joseluis8906/protobuf/delivery/storemanager/storemanager.proto"],
            &["proto/joseluis8906"],
        )?;

        tonic_build::configure()
        .out_dir("src/pb/tonicpb")
        .type_attribute(
            "delivery.Store",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .field_attribute(
            "delivery.Store.products",
            "#[serde(skip_serializing_if = \"Vec::is_empty\", default)]",
        )
        .type_attribute(
            "delivery.Product",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "delivery.Money",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .build_server(false)
        .compile(
            &["proto/joseluis8906/protobuf/delivery/storemanager/storemanager.proto"],
            &["proto/joseluis8906"],
        )?;

        Ok(())
}
