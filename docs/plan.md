# FOSSil Development Plan

## 1. Phase 1: System Design
Goal: Define how everything works before building features.

Decide:
- Message format (JSON or binary like bincode)
- User identity system
- Authentication approach
- Encryption strategy (later phase)
- Server model (centralized first, federation later)

Output:
- architecture documentation
- protocol specification
- encryption plan

---

## 2. Phase 2: Core Chat (No Accounts)
Goal: Working chat system as fast as possible.

Features:
- Server accepts multiple TCP connections
- Broadcast messages to all clients
- Basic connection handling
- Simple logging

Result:
- Multiple users can chat in real time

---

## 3. Phase 3: CLI Client
Goal: Usable terminal chat client.

Features:
- Connect to server
- Send messages
- Receive and display messages
- Basic input loop

Result:
- Functional terminal chat application

---

## 4. Phase 4: Multi-User System
Goal: Make chat structured.

Features:
- Usernames
- Join/leave notifications
- Private messages
- Message routing improvements

Result:
- Real multi-user chat environment

---

## 5. Phase 5: Accounts + Persistence
Goal: Persistent identities.

Features:
- User registration/login
- Password hashing (argon2 or bcrypt)
- Simple storage (SQLite or file-based DB)
- Session handling

Result:
- Users keep identities across sessions

---

## 6. Phase 6: Encryption Layer
Goal: Secure communication.

Features:
- Key exchange (X25519)
- Message encryption (ChaCha20-Poly1305)
- Server only relays encrypted data
- No plaintext message access on server

Result:
- End-to-end encrypted messaging

---

## 7. Phase 7: Rooms and Groups
Goal: Expand beyond 1 global chat.

Features:
- Chat rooms / channels
- Group chats
- Permissions (admin, moderator, user)

Result:
- Structured chat communities

---

## 8. Phase 8: Attachments
Goal: Support richer content.

Features:
- File transfer
- Image sharing
- Message metadata
- Size limits and validation

Result:
- Media-capable chat system

---

## 9. Phase 9: GUI Client
Goal: Replace terminal client.

Options:
- egui (fastest to build)
- Iced
- Slint

Features:
- Chat window
- Rooms sidebar
- Notifications
- Settings panel

Result:
- Desktop chat app

---

## 10. Phase 10: Community-Hosted Servers
Goal: Anyone can run a server.

Features:
- Configurable server settings
- Admin controls
- Logging system
- Optional registration mode (open/closed)
- Stable release binaries

Result:
- Users can host their own servers easily

---

## 11. Phase 11: Federation (Advanced)
Goal: Connect independent servers.

Features:
- Server-to-server communication
- Cross-server messaging
- Identity mapping
- Federation routing rules

Result:
- Network of independent chat servers

---

## 12. Phase 12: Polish + Production Features
Goal: Make it feel like a real product.

Features:
- Typing indicators
- Read receipts
- Reactions
- Message editing/deletion
- Search
- Notifications
- Rate limiting
- Spam protection

Result:
- Polished chat platform

---

## 13. Phase 13: Release v1.0
Goal: Public launch.

Deliverables:
- Client binaries
- Server binaries
- Documentation site
- Installation guide
- Contribution guide
- Public roadmap

---

## Development Strategy

Build in this order:
1. Server first (basic networking)
2. CLI client next
3. Multi-user chat working
4. Then features one by one

Rule:
If a phase breaks the system, do not continue. Fix before moving on.

---

## Core Principle

Keep the system working at all times:
- Every phase must end with a usable chat system
- Avoid building encryption or federation early
- Complexity is added only after stability
