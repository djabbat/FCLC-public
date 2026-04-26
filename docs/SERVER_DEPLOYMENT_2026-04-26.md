# FCLC Server Deployment — fclc.longevity.ge

**Дата:** 2026-04-26
**Триггер:** user request «fclc nujno perenesti na server i zupustit v fclc.longevity.ge»
**Server:** `jaba@server` (77.42.67.59 / 2a01:4f9:c013:2b22::1)
**Domain:** longevity.ge (subdomain fclc.*)

---

## Status overview

| Compoнент | Готов | Пояснение |
|---|---|---|
| Server SSH access | ✅ | `ssh server` работает |
| Rust/cargo on server | ✅ | `/home/jaba/.cargo/bin/cargo` |
| Docker | ✅ | `/usr/bin/docker` |
| docker-compose | ⚠️ | not found; используем `docker compose` plugin или install `docker-compose` package |
| nginx | ✅ | `/usr/sbin/nginx` running |
| certbot (SSL) | ❌ | needs install: `apt install certbot python3-certbot-nginx` |
| **DNS fclc.longevity.ge** | ❌ | **БЛОКЕР** — нужна A/AAAA запись |
| nginx config для fclc | ❌ | нужно создать `/etc/nginx/sites-available/fclc.longevity.ge` |
| FCLC code на server | ❌ | пока на Desktop, нужен deploy |
| TLS cert | ❌ | depends на DNS resolved |

---

## Pre-deployment (до DNS) — что можно сделать сейчас

1. ✅ **Copy FCLC code на server** в `/home/jaba/web/fclc/`
2. ✅ **Build cargo на server** — verify все crates компилируются
3. ✅ **Подготовить nginx config (disabled)** — готов к активации после DNS
4. ✅ **Подготовить systemd unit / docker-compose**
5. ⏸ **Activate nginx + SSL** — после DNS resolves

---

## Действия пользователя (DNS) — БЛОКЕР

**При вашем registrar / Cloudflare для longevity.ge добавить:**

```
Type: A
Name: fclc
Content: 77.42.67.59
TTL: Auto (или 300s)
Proxy: Yes (если Cloudflare-proxied как drjaba.com)

Type: AAAA
Name: fclc
Content: 2a01:4f9:c013:2b22::1
TTL: Auto
Proxy: Yes
```

После этого `dig +short fclc.longevity.ge` должен возвращать IP.

---

## Архитектура deployment

```
[User browser] 
    ↓ HTTPS
[Cloudflare CDN] (TLS-termination + DDoS)
    ↓ HTTP
[nginx server :443/80]
    ↓ proxy_pass http://127.0.0.1:4002
[fclc-server (Rust axum)] — main API
    ↓ secret aggregation
[fclc-node × N] — federated nodes
    ↓
[Postgres database]
```

**Port assignment:**
- fclc-server: `127.0.0.1:4002` (главный API; остальные порты заняты: 4000=space, 4001=spellcheckerka, 8080=drjaba)
- fclc-node ports: 5001-5010 (если на one server для testing)

---

## FCLC components (из `/home/oem/Desktop/LongevityCommon/FCLC/`)

| Component | Cargo crate | Purpose | Deploy strategy |
|---|---|---|---|
| `fclc-core` | shared types | core protocol structs | depend |
| `fclc-server` | axum service | main coordinator | systemd unit |
| `fclc-node` | client lib | participant node | optional, for testing |
| `fclc-demogen` | data generation | synthetic data | dev tool, не deploy |
| `fclc-web` | Yew/Leptos frontend | UI | static build → nginx |

---

## Deployment plan (пошагово)

### Step 1: rsync code to server

```bash
ssh server "mkdir -p /home/jaba/web/fclc"
rsync -av --delete \
  --exclude='target/' --exclude='node_modules/' \
  --exclude='.git/' --exclude='_archive/' \
  --exclude='docs/' \
  /home/oem/Desktop/LongevityCommon/FCLC/ \
  server:/home/jaba/web/fclc/
```

### Step 2: build на server

```bash
ssh server "cd /home/jaba/web/fclc && cargo build --release --workspace"
```

**Expected build time:** 5-15 мин на server (depends на CPU + cache).

### Step 3: подготовить nginx config

`/etc/nginx/sites-available/fclc.longevity.ge`:

```nginx
server {
    listen 80;
    listen [::]:80;
    server_name fclc.longevity.ge;
    
    # Letsencrypt challenge path (для certbot)
    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }
    
    location / {
        return 301 https://$server_name$request_uri;
    }
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name fclc.longevity.ge;
    
    # SSL (после certbot)
    # ssl_certificate /etc/letsencrypt/live/fclc.longevity.ge/fullchain.pem;
    # ssl_certificate_key /etc/letsencrypt/live/fclc.longevity.ge/privkey.pem;
    
    # Static frontend (Yew/Leptos build)
    location / {
        root /home/jaba/web/fclc/fclc-web/dist;
        try_files $uri $uri/ /index.html;
    }
    
    # API
    location /api/ {
        proxy_pass http://127.0.0.1:4002;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Federated learning: long-poll friendly
        proxy_read_timeout 300s;
        proxy_buffering off;
    }
    
    # WebSocket (если есть)
    location /ws/ {
        proxy_pass http://127.0.0.1:4002;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 86400s;
    }
}
```

### Step 4: SSL via certbot

```bash
ssh server "sudo apt install -y certbot python3-certbot-nginx"
ssh server "sudo certbot --nginx -d fclc.longevity.ge"
```

### Step 5: systemd unit для fclc-server

`/etc/systemd/system/fclc-server.service`:

```ini
[Unit]
Description=FCLC Federated Clinical Learning Server
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=jaba
WorkingDirectory=/home/jaba/web/fclc
Environment=RUST_LOG=info
Environment=DATABASE_URL=postgresql://fclc:CHANGEME@localhost/fclc
Environment=BIND_ADDR=127.0.0.1:4002
ExecStart=/home/jaba/web/fclc/target/release/fclc-server
Restart=on-failure
RestartSec=10s

[Install]
WantedBy=multi-user.target
```

Activate:
```bash
sudo systemctl daemon-reload
sudo systemctl enable fclc-server
sudo systemctl start fclc-server
sudo systemctl status fclc-server
```

### Step 6: Postgres database

```bash
ssh server "sudo -u postgres createdb fclc"
ssh server "sudo -u postgres createuser fclc -P"  # set password
# Initialize schema
ssh server "cd /home/jaba/web/fclc && cargo run --bin fclc-server -- migrate"
```

### Step 7: enable nginx site + restart

```bash
ssh server "sudo ln -s /etc/nginx/sites-available/fclc.longevity.ge /etc/nginx/sites-enabled/"
ssh server "sudo nginx -t && sudo systemctl reload nginx"
```

---

## Verification

После deployment:
- [ ] `curl https://fclc.longevity.ge/api/health` → 200 OK
- [ ] `curl https://fclc.longevity.ge/` → static UI
- [ ] `systemctl status fclc-server` → active (running)
- [ ] `journalctl -u fclc-server --since "1h ago"` → no errors
- [ ] DNS `dig fclc.longevity.ge` → returns server IP
- [ ] SSL cert valid: `openssl s_client -connect fclc.longevity.ge:443`

---

## Risks + mitigations

| Risk | Mitigation |
|---|---|
| Port 4002 conflict с другим service | Pre-checked — used 4000/4001/8080 free; 4002 free |
| Build failure on server (Rust version mismatch) | Use `rustup show` сheck — установить `1.77.0+` если нужно |
| DNS propagation delay 24-48ч | Wait or use Cloudflare proxy (instant) |
| nginx config conflict с longevity.ge OJS | Separate file per subdomain — no conflict |
| FCLC frontend (fclc-web) not yet built | Use placeholder index.html сначала, добавить web позже |
| Database migrations missing | Manual schema setup if migrate command not implemented |

---

## Status: Step 1 — IN PROGRESS

Pre-deployment staging запущен (rsync + cargo build на server). DNS — pending user action. После DNS ready — финальные шаги (4-7) возьмут ~30 мин.
