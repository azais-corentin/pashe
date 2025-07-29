<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a id="readme-top"></a>

<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![GNU GPLv3 License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/azais-corentin/pashe">
  </a>

<h3 align="center">pashe</h3>

  <p align="center">
    A Path of Exile trade analysis tool with a Rust backend and a Svelte/Tauri frontend.
    <br />
    <a href="https://github.com/azais-corentin/pashe"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/azais-corentin/pashe">View Demo</a>
    &middot;
    <a href="https://github.com/azais-corentin/pashe/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com/azais-corentin/pashe/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

This repository contains the source code for Pashe, a tool for analyzing the Path of Exile economy. It is composed of three main parts:

- `pashe-frontend`: A desktop application built with Svelte and Tauri. This is the user-facing part of the application.
- `pashe-backend`: A server application written in Rust. It fetches data from the official Path of Exile public stash tab API to estimate item prices, much like poe.ninja. The goal is to perform statistical analysis on the game's economy and find interesting trends.
- `db`: A small utility tool for database management.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

[![Rust][Rust.js]][Rust-url]
[![Svelte][Svelte.js]][Svelte-url]
[![Tauri][Tauri.js]][Tauri-url]
[![Docker][Docker.js]][Docker-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

To get a local copy up and running follow these simple example steps.

### Prerequisites

It is recommended to use the provided devcontainer for development, which comes with all the necessary dependencies for both backend and frontend development.

If you choose not to use the devcontainer, you will need to install the following dependencies manually:

- Rust and Cargo
- bun

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/azais-corentin/pashe.git
   ```
2. Build the project
   ```sh
   cargo build --release
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Top contributors:

<a href="https://github.com/azais-corentin/pashe/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=azais-corentin/pashe" alt="contrib.rocks image" />
</a>

<!-- LICENSE -->

## License

Distributed under the GNU GPLv3 License. See [`LICENSE`](https://github.com/azais-corentin/pashe/blob/main/LICENSE) for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Corentin Azais - azaiscorentin@gmail.com

Project Link: [https://github.com/azais-corentin/pashe](https://github.com/azais-corentin/pashe)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/azais-corentin/pashe.svg?style=for-the-badge
[contributors-url]: https://github.com/azais-corentin/pashe/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/azais-corentin/pashe.svg?style=for-the-badge
[forks-url]: https://github.com/azais-corentin/pashe/network/members
[stars-shield]: https://img.shields.io/github/stars/azais-corentin/pashe.svg?style=for-the-badge
[stars-url]: https://github.com/azais-corentin/pashe/stargazers
[issues-shield]: https://img.shields.io/github/issues/azais-corentin/pashe.svg?style=for-the-badge
[issues-url]: https://github.com/azais-corentin/pashe/issues
[license-shield]: https://img.shields.io/github/license/azais-corentin/pashe.svg?style=for-the-badge
[license-url]: https://github.com/azais-corentin/pashe/blob/master/LICENSE
[Rust.js]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Svelte.js]: https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white
[Svelte-url]: https://svelte.dev/
[Tauri.js]: https://img.shields.io/badge/Tauri-24C8DB?style=for-the-badge&logo=tauri&logoColor=white
[Tauri-url]: https://tauri.app/
[Docker.js]: https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white
[Docker-url]: https://www.docker.com/
