# DesignAdobado

<p align="center"><img src="assets/adobado/icon.png" width="104" alt="DesignAdobado"></p>

**Diseno y composicion local.** Derivado de DesignCraft, rebrandeado para Adobado Kit.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-6cb6ff)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/Rust-1.95%2B-ff7a59)](Cargo.toml)
[![Windows build](https://github.com/smouj/DesignAdobado/actions/workflows/windows-build.yml/badge.svg?branch=main)](https://github.com/smouj/DesignAdobado/actions/workflows/windows-build.yml)
[![Estado](https://img.shields.io/badge/status-instalaci%C3%B3n%20pendiente-ff7a59)](#estado)
[![Estrellas](https://img.shields.io/github/stars/smouj/DesignAdobado?style=flat&color=B8F34A)](https://github.com/smouj/DesignAdobado/stargazers)
[![Adobado Kit](https://img.shields.io/badge/Adobado%20Kit-hub-b8f34a)](https://github.com/smouj/Adobado_Kit)

> **Build de Windows correcta (20/10/2026).** Falta instalar e iniciar la app manualmente. La prueba de abrir, modificar y exportar un archivo real también está pendiente. La imagen siguiente es una tarjeta de marca, no una captura de la interfaz.
>
> Proyecto independiente. No afiliado a ArtCraft ni a Adobe.

![DesignAdobado — tarjeta de marca; interfaz pendiente](docs/preview.png)

<sub>Tarjeta de marca; no es una captura de la interfaz.</sub>

## Estado

- **Build de Windows:** correcto en GitHub Actions; consulta el [workflow actual](https://github.com/smouj/DesignAdobado/actions/workflows/windows-build.yml).
- **Prueba manual:** instalación e inicio pendientes.
- **Flujo funcional:** abrir, modificar y exportar un archivo real sigue pendiente.
- **Disponibilidad:** alpha; todavía no se declara lista para uso general.
- **Release pública:** todavía no existe. No se publica un binario hasta completar las pruebas.
- **Commit de origen:** 443ea4640ca368fcf0241b04813bd3817acd5df8.

## Build

Requisitos: Rust 1.95 o superior.

    cargo build --release -p designcraft

El nombre interno del crate se conserva para no romper el workspace; el nombre de producto visible es DesignAdobado.

## Procedencia y licencias

- Upstream: https://github.com/storytold/designcraft
- Commit base: 443ea4640ca368fcf0241b04813bd3817acd5df8
- Licencia: MIT OR Apache-2.0. Se conservan los archivos de licencia, NOTICE y ATTRIBUTION del upstream.
- Cambios: docs/ADOBADO.md.
- La marca, el wordmark y los logos de ArtCraft no estan cubiertos por la licencia y se retiran de la distribucion.

## Adobado Kit

Este repo forma parte de Adobado Kit: https://github.com/smouj/Adobado_Kit

Hecho por SMOUJ013.
