# Setup

## 1. Install Rust

On Unix, Mac and Github Codespaces you can use:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Install Docker

This is dependent on your OS
Github Codespaces has Docker preinstalled

## 3. Deploy an instance of KubeMQ with Docker

```bash
docker run -d -p 8080:8080 -p 50000:50000 -p 9090:9090 -e kubemq/kubemq
```
