## GitMD

Very work in progress, as I'm still learning rust.
However I feel specifically for this project that running in a "safe environment" is a good idea.
Along with that the speed benefit of rust is a good thing for bringing down costs of deployment.

The idea of this project is to create something of a medical record system built on top of git.
Each patient will have their own repository as this is rather scalable, and will allow for easy version control, collaboration and data backup and retention mechanisms.

Every patient will have a `README.md` file, which will contain an overview of the patient's medical history.

All control when it comes to Git itself will still be available to administrators, but the main purpose of this project is to provide a simple way to store and retrieve medical data. While allowing for easy version control and collaboration.

The Git frontend would be highly damaging to the user experience, but it is still possible to use the Git backend to store and retrieve data while providing a rich user experience, automating commits and other features of git, like merging &amp; merge conflicts. As well as using the tree-like structure of git to organize data and provide an at a glance view of the patient history.