from __future__ import annotations
import re
from pathlib import Path
from .config import BUCKETS, CONFIG_FILE, DISCOVERY_STEMS, ROOT_FALLBACK_FILES, Context, Paths

def _natural_key ( path: Path ) -> list:

    return [int(part) if part.isdigit() else part.lower() for part in re.split(r"(\d+)", path.name)]

def _config_in ( directory: Path ) -> Path | None:

    if not directory.is_dir():
        return None

    target = CONFIG_FILE.lower()

    for child in directory.iterdir():

        if child.is_file() and child.name.lower() == target:
            return child

    return None

def resolve_root ( start: Path ) -> Path:

    start = start.resolve()

    if _config_in(start):
        return start

    for parent in ( start, *start.parents ):

        if (parent / ".git").exists():
            return parent

    for parent in ( start, *start.parents ):

        if _config_in(parent):
            return parent

    return start

def _bucket_of ( stem: str ) -> str | None:

    for bucket, stems in DISCOVERY_STEMS.items():

        if stem in stems:
            return bucket

    return None

def discover ( paths: Paths ) -> Context:

    context = Context()

    if paths.docs.is_dir():

        for entry in sorted(paths.docs.iterdir()):

            if entry.is_file():

                if entry.suffix.lower() != ".md":
                    continue

                bucket = _bucket_of(entry.stem.lower())

                if bucket:
                    context.add(bucket, entry)

            elif entry.is_dir():

                bucket = _bucket_of(entry.name.lower())

                if bucket:

                    for md in entry.rglob("*.md"):

                        if md.is_file():
                            context.add(bucket, md)

    else:

        for child in sorted(paths.root.iterdir()):

            if child.is_file() and child.name.lower() in ROOT_FALLBACK_FILES:
                context.add("overview", child)

    for name in BUCKETS:
        context.bucket(name).sort(key=_natural_key)

    return context
