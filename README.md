<h1 align="center">VSN-Tournament Manager</h1>
<h3 align="center">Organizing tournament and leagues in a effiencient and easy manner</h3>

<div align="center">
<a href="#getting-started">Getting started</a> •
<a href="#contributing">Contributing</a> •
<a href="#documentation">Documentation</a> •
<a href="license">License</a>
</div>

## Getting started:

### Step 1

Before running the project, you'll need to have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo: Cargo comes with Rust if you installed it from the official website
- [MySQL](https://dev.mysql.com/downloads/installer/)

### Step 2

Once you have these tools installed, you can clone the repository and navigate into the project directory:

  

```bash
git  clone  https://github.com/MatheusVSN/vsn-tournament-manager.git
cd  vsn-tournament-manager
```

### Step 3

Inside the project, there is a folder named `database` which contains a file named `main.sql`.
You need to execute this file in your MySQL to set up the database for the project.
You can do this by running the following command in your terminal (replace yourusername and yourpassword with your MySQL username and password):

```bash
mysql  -u  yourusername  -p  yourpassword  <  database/main.sql
```

### Step 4

Please note that you may need to configure your MySQL database connection in a .env file in the root of your project.

Refer to the `.env.example` file in the repository for an example of how to set this up.

### Step 5

After setting up the database and the enviroments variables, you can build and run the project using Cargo:

```bash
cargo  run
```

## Contributing

If you want to contribute to this project, you can fork the repository and create a pull request.

You can also open an issue if you find any bugs or have any suggestions.

## Documentation

For more details about the project and its usage, please refer to the [Documentation](https://matheusvsn.github.io/vsn-tournament-manager/).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details
