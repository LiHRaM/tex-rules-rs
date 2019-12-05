# tex-rules-rs
Simple CI style tool for .tex files.
Currently, it just checks whether you keep a 1:1 ratio between sentences and lines.

## Using
Use the docker.io/lihram/tex-rules image, running tex_rules in the directory where you keep your .tex files.
The following is an excerpt from my .gitlab.ci.yml file.

```yaml
tex_rules:
  stage: lint
  image: lihram/tex-rules
  before_script:
    - cd report
  script:
    - tex_rules
```
