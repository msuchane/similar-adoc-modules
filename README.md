# similar-adoc-modules

Identify modules in Red Hat documentation that are too similar, or identical. Compares text files using the Levenshtein distance metric.

## Usage

1. On Fedora, RHEL, or CentOS, install this program from the following repository: <https://copr.fedorainfracloud.org/coprs/mareksu/similar-adoc-modules/>.

2. Run this program at the root of the documentation repository.

3. The program continually prints out file information to the terminal. Finally, it saves all statistics sorted by file similarity to the `comparisons.csv` file in the current directory.
