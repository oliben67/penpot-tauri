# penpot-electron-app

An [Electron](https://www.electronjs.org/) wrapper for [Penpot](https://penpot.app) that targets a **local self-hosted instance** instead of the cloud service.

![Screenshot](https://github.com/vikdevelop/penpot/blob/main/screnshots/penpot_home.png)

## Source repositories

| Component | Source | Branch/Path |
|-----------|--------|--------------|
| Electron app (`src/`, `package.json`) | [vikdevelop/penpot-electron](https://github.com/vikdevelop/penpot-electron) | `main` |
| Docker Compose stack (`docker-compose.yml`) | [penpot/penpot](https://github.com/penpot/penpot) | `main` › `docker/images/docker-compose.yaml` |

The `docker-compose.yml` in this repository carries its full upstream git history
(46 commits) imported from `penpot/penpot` via an orphan-branch merge, so
`git log -- docker-compose.yml` traces every change back to the source.

## Local instance quick-start

Requires: `docker`, `node` (≥ 16), `npm`, `nc` (netcat).

```bash
# clone and run in one step
git clone git@github.com:oliben67/penpot-electron-app.git
cd penpot-electron-app
bash start.sh
```

The script will:
1. Start the full Penpot stack via `docker compose`
2. Wait for the frontend to be reachable on `localhost:9001`
3. Install npm dependencies
4. Launch the Electron window pointing at `http://localhost:9001`

To use a different URL or port:
```bash
PENPOT_URL=http://localhost:9002 bash start.sh
```

## Flatpak installation (original upstream method)

For the original cloud-hosted build via Flatpak:
```bash
wget -qO /tmp/build.sh https://raw.githubusercontent.com/vikdevelop/penpot-electron/main/build.sh && sh /tmp/build.sh
```

Dependencies:
```bash
flatpak install -y org.electronjs.Electron2.BaseApp org.freedesktop.Sdk org.freedesktop.Sdk.Extension.node16
```
