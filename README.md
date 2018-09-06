# horaires_uclouvain
Parce que je ne connais pas mon horaire par coeur et que le nouveau portail ne m'aide pas à le trouver facilement..

```
horaires
Vous redirige vers la page ADE avec *votre* horaire.

USAGE:
    horaires [COURSES]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <COURSES>    La liste de vos cours séparés par une virgule. (Env.Var $COURSES p/ defaut)

```

# Customisation
Pour customiser votre installation (cours par défauts quand vous ne spécifiez aucun argument), il vous suffit de créer un fichier `.env` avec le contenu suivant (ou de placer ce contenu dans votre `.profile`)

```
# This is the configuration to the current ADE instance
horaires_protocol=http
horaires_hostname=horaire.uclouvain.be
horaires_path=/direct/index.jsp
horaires_login=etudiant
horaires_password=student

# This is the list of courses you have to teach
horaires_courses=LINGI2255,LINGI2143,LINFO1103,LINGI1122
```
