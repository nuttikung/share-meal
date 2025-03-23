# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform web
dx serve --platform ios
dx serve --platform desktop
```

For development use command for tailwind

Installation
```bash
npm install tailwindcss @tailwindcss/cli
```

To watch and compile css
```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```
