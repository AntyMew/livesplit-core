import os
from setuptools import setup

setup(name="livesplit-core",
				 version="0.9.0",
				 description="livesplit-core is a library that provides a lot of functionality for creating a speedrun timer.",
				 url="https://github.com/LiveSplit/livesplit-core",
				 author="Christopher Serr",
				 author_email="christopher.serr@gmail.com",
				 maintainer="AntyMew",
				 maintainer_email="antylamon413@gmail.com",
				 license="MIT",
				 packages=["livesplit_core"],
				 package_data={"livesplit_core": ["lib/liblivesplit_core.so"]},
				 include_package_data=True)