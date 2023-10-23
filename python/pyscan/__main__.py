import os
import sys
import sysconfig
from pathlib import Path


def find_pyscan_bin() -> Path:
    """Return the pyscan binary path."""

    pyscan_exe = "pyscan" + sysconfig.get_config_var("EXE")

    path = Path(sysconfig.get_path("bin")) / pyscan_exe
    if path.is_file():
        return path

    if sys.version_info >= (3, 10):
        user_scheme = sysconfig.get_preferred_scheme("user")
    elif os.name == "nt":
        user_scheme = "nt_user"
    elif sys.platform == "darwin" and sys._framework:
        user_scheme = "osx_framework_user"
    else:
        user_scheme = "posix_user"

    path = Path(sysconfig.get_path("bin", scheme=user_scheme)) / pyscan_exe
    if path.is_file():
        return path

    raise FileNotFoundError(path)


if __name__ == "__main__":
    pyscan = find_pyscan_bin()
    sys.exit(os.spawnv(os.P_WAIT, pyscan, ["pyscan", *sys.argv[1:]]))