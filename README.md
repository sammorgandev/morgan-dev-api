# API Project

## Overview

This project is an API written in Rust. It is designed to be efficient and scalable, leveraging Rust's powerful concurrency capabilities and type safety. This README provides detailed instructions on how to set up, run, and contribute to this project.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Endpoints](#endpoints)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Authentication**: Secure user authentication.
- **Email Service**: Send emails via the API.
- **Post Management**: CRUD operations for posts.
- **Database Connection**: Efficient database handling with connection pooling.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Docker (for running the database in a container)

## Installation

1. **Clone the Repository**

   ```sh
   git clone https://github.com/sammorgandev/morgan-dev-api.git
   cd morgan-dev-api
   ```

2. **Install Dependencies**
   Ensure you have Rust installed. If not, install it from rust-lang.org.

   ```sh
   cargo build
   ```

3. **Set up PostgreSql**
   You can use Docker to set up a PostgreSQL instance:
   ```sh
   docker run --name postgres -e POSTGRES_PASSWORD=yourpassword -d -p 5432:5432 postgres
   ```

## Configuration

Update the .env file with your database credentials and other configuration details:

    ```env
    POSTGRES_USER=your database user
    POSTGRES_PASSWORD=your database password
    POSTGRES_DB=your database name
    POSTGRES_HOSTNAME=your database hostname
    POSTGRES_PORT=5432
    API_PORT=8080
    FRONTEND_PORT=3000
    FRONTEND_URL=http://localhost:3000
    API_URL=http://localhost:8080
    LOOPS_API_KEY=your loops (or other email provider's) api key
    API_USERNAME=your admin username
    API_PASSWORD=your admin password
    API_SECRET=a custom secret for authentication
    ```

## Usage

Run this API server with:

    ```sh
    cargo run
    ```

The server will start on http://localhost:8080. You can change the port in your .env file.

## License

This project is licensed under the MIT License.

    ```license
    Copyright 2024 MORGAN/DEV

    Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
    ```
