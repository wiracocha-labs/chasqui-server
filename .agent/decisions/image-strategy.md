# 🖼️ Image Management Strategy

This document outlines the phased approach for managing images in the Chasqui Server project.

## 📋 Architecture: Direct URL

Images are stored as files on disk and the database only stores URLs (strings). The frontend accesses images directly without going through the backend.

```
Backend → Saves image → Returns URL string
Frontend → Uses URL directly in <img src="url">
```

### Advantages of this architecture:
- ✅ **Performance**: Web server serves static files very fast.
- ✅ **Automatic caching**: Browser and CDN cache without additional configuration.
- ✅ **Scalability**: Doesn't overload backend with byte transfer.

---

## 🚀 Implementation Roadmap

### 🟢 PHASE 1: MVP / Development (Local Files)
- Folder `./uploads/` on server.
- `actix-files` serves static content.
- DB stores URLs as strings.
- **Cost**: $0

### 🟡 PHASE 2: Growth (CDN Integration)
- Connect Cloudflare CDN to the `/uploads/` path.
- No code changes needed.
- **Benefits**: Global caching, DDoS protection.

### 🔴 PHASE 3: Scale (Cloudflare R2)
- Migrate to R2 (S3-compatible) when storage exceeds 200GB.
- Use `aws-sdk-s3` for integration.
- **Benefits**: Infinite scaling, automatic backups, $0 egress fees.

---

## 📝 Implementation Example (Phase 1)

Refer to the original implementation plan for Actix-web multipart and static file serving code snippets.
