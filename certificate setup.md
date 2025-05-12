# TLS Certificate Setup for Rust HTTP Server

This guide explains how to generate a self-signed TLS certificate with OpenSSL for your Rust HTTP server, supporting multiple IP addresses via SAN (Subject Alternative Names).

## Prerequisites
- OpenSSL: Install OpenSSL if you don’t have it

## 0. Init
Try to keep your certs and config files in separate directory within this repo.
```
rust-http-server/
├── certs/
│   ├── cert.crt 
│   ├── key.pem
│   └── custom_config.cnf
└── src/
...
```
## 1. Config
Create or modify the custom_config.cnf file to include SANs for your IPs and localhost.

```
[ req ]
default_bits        = 2048
distinguished_name  = req_distinguished_name
req_extensions      = v3_req
prompt              = no

[ req_distinguished_name ]
CN = localhost

[ v3_req ]
subjectAltName = @alt_names

[ alt_names ]
DNS.1 = localhost # not needed really
IP.1 = 127.0.0.1
# IP.n = 192.168.x.x -> Add your local IP here
```
>Note: Add more IPs under [alt_names] as needed.

## 2. Generate certificate
Run this script to generate certificate
```
openssl req -x509 -nodes -days 365 \
  -newkey rsa:2048 \
  -keyout key.pem \
  -out cert.crt \
  -config custom_config.cnf \
  -extensions v3_req
```

## 3. Install the certificate
Follow these steps to install your new certificate on __Windows__
1. Locate the cert.crt File in your file manager
2. Double-Click the Certificate
3. Install the Certificate
   1. After double-clicking, a window should pop up with certificate details
   2. Click Install Certificate
   3. Choose Local Machine and click Next
   4. Select Place all certificates in the following store and choose Trusted Root Certification Authorities
   5. Click Finish and then confirm the installation

## 4. Restart your browser

## 5. Finish line
Now, when you are creating an instance of this webserver, you can use ```with_tls()``` method and provide it paths to your certificate and key files

<hr>
By installing the certificate this way, your machine (and browser) will trust your self-signed certificate when you connect to your server via https://localhost or any other IP listed in the SAN