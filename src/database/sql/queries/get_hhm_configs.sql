SELECT
    sys.id,
    sys.manufacturer,
    sys.modality,
    ac.host_ip,
    ac.vpn,
    ac.acqu_point,
    ac.debian_server_path,
    ac.credentials_group,
    ac.acquisition_script,
    ac.host_path,
    ac.cerb_file,
    crd.user_enc,
    crd.password_enc
FROM
    systems sys
    JOIN config.acquisition ac ON sys.id = ac.system_id
    JOIN hhm_credentials_rust crd ON crd.id = ac.credentials_group::int
WHERE
    sys.manufacturer = $1
    AND sys.modality LIKE $2
    AND sys.process_log = true
    AND sys.id IN ('SME02412');

    -- GE_CT ('SME17378', 'SME17368', 'SME17377');
    -- GE CV ('SME02412', 'SME00865', 'SME16399');