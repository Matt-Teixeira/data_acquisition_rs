SELECT
    *
FROM
    util.ip_sec
WHERE
    remote_subnet_ip = ANY($1)
ORDER BY
    remote_subnet_ip;