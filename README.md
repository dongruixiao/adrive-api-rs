# Rust implementation of Aliyundrive API

I am a Rust beginner, and this is my first project in Rust development, which is not yet available. Its progress is slow, and my learning progress is also slow, but it will be continuously updated.

## Overview Diagram

I have actually done some development-related work. I want to do some small refactoring at this stage, but I can't see the whole picture of this project completely yet. However, I am trying to explain what this project may accomplish with the following diagram.

```mermaid
flowchart TB
    Album --> API
    User --> API
    Subscription --> API
    FileEntry --> File
    DirEntry --> File
    File --> API
    SafeBox --> API
    Starred --> API
    RecyleBin --> API


    Signin & Refresh--> AccessToken & RefreshToken --> Credentials 

    HttpClient --> Request
    Metadata --> Request --> FileEntry & DirEntry & User & SafeBox & Starred & RecyleBin & Album & Subscription

    DeviceID & UserId & AppId & nonce --> Signature
    DriveId & Signature  --> Metadata
    Credentials --> Metadata
    Constants --> Request

    API --> Response
```
