---
date: 2024-08-09T22:51:00Z
title: "Weebo GitOps"
description: |
    Weebo GitOps est un homelab iteratif orienté GitOps. C'est mon homelab qui évolue au fil du temps et qui me sert à apprendre de nouvelles technologies et à tester de nouvelles choses.
spec:
  blog: true
  project: true
  doc: true
tags:
 - project
 - Kube
 - continuous-update
 - GitOps
 - homelab
techno:
  - Kubernetes
  - ArgoCD
  - Helm
  - Terraform
  - Gitea
  - Kustomize
---

## Weebo GitOps

Ce projet dure depuis 2018 avec la première itération Weebo 1.0. Depuis, sa forme a beaucoup évolué, passant d'un simple serveur où j'ai pue installer mes premières LAMP assorties de serveur minecraft a un cluster kubernetes complet avec une approche GitOps.

### Weebo 1.0 - Weebo 2.0 - 2018 => 2023

Weebo 1.0 était mon premier serveur baremetal OVH. Celui-ci a porté de nombreux outils et projets, a l'époque pas d'automatisation, pas de GitOps, juste de bon vieux tuto sur comment installer un LAMP, comment lancer une instance minecraft à l'aide de screen pour ne pas couper le serveur en quittant le terminal. Beaucoup de mes TP en IUT info ont été faits sur ce serveur, c'était mon premier pas dans le monde de l'administration système. Au cours de cette période, j'ai même fait une passe par [IspConfig](https://www.ispconfig.org/) pour faciliter la gestion des sites web / Mail / DNS. Un bon vieux panel de contrôle qui a fait le taf pour ce que je faisais a l'époque.

Weebo 2.0 a vu mes premiers pas sur Docker, j'ai commencé à containériser mes applications, déployer une stack Nginx pour proxyfier mes applications. C'est aussi a ce moment que j'ai commencé a automatisé mes déploiements, pousser mes configurations sur un dépôt git, mais a des années lumières du GitOps. Encore aujourd'hui, je retrouve le repos à l'origine de cette époque qui n’a plus bougé depuis 2022, c'est un peu la capsule temporelle de mes débuts dans l'automatisation et la containérisation. (À partager un jour, peut-être, son petit nom ? WeeboServeur).

C'est aussi a l'époque de Weebo 1.0 que j'ai découvert [Code Server](https://github.com/coder/code-server) qui est-on peut le dire mon début avec les CDE (Cloud Development Environment).

### Weebo 3 - février 2021 => mai 2026

#### Weebo 3.0

Weebo 3.0 a été le successeur de Weebo 2.0, parsemé de conteneurisation hasardeuse, un DNS sous CoreDNS sans pour autant de cluster Kube. Des CI/CD sous [Drone](https://www.drone.io/)/GitHub Action (avec des runner SelfHosted) / Gitlab CI (toujours avec des runner maison), mais surtout, mon petit préféré de l'époque [Concourse CI](https://github.com/concourse/concourse) que j'ai autant adoré que détesté (Bordel tu m'aura couter des nuits blanches toi !!).

Mais surtout, peu après le passage a Weebo 3.0, j'ai commencé à mettre les mains dans Kubernetes à travers mon premier nœud [MicroK8S](https://github.com/canonical/microk8s). C'était un cluster mononoeud, pas de haute disponibilité, pas de GitOps, juste un kube pour apprendre et tester des choses. Il a aussi été mon plus gros cauchemar, a cette époque, si tu restart le nœud, impossible de remonter les pods à moins d'avoir un autre nœud dans le cluster. Il fut très rapidement remplacé par [K3S](https://k3s.io/) qui, lui, a été un vrai plaisir à utiliser, pas de soucis, léger, rapide, facile à mettre en place. Il n'a été remplacé en tén que cluster de production que le 1er mai 2026 lors de la résiliation de Weebo 3.0. Mais c'est une histoire pour plus tard.

C'est aussi avec l'arrivée de K3S que j'ai commencé à mettre en place Ansible pour l'automatisation de la configuration de mes nœuds.

Cette itération a aussi été marqué par l'utilisation d'un CDE, dans un premier temps toujours Code Server, puis plus tard [Gitpod](https://www.gitpod.io/) qui commençait tout juste a se faire une place dans ce monde émergeant. Il a été récemment rebandé en [Ona](https://ona.com/stories/gitpod-is-now-ona) et doté de fonctionnalités orientées IA.

### Weebo 3.1

Avec la fin de Weebo 3.0 en 2022, l'approche GitOps a commencé a prendre place avec l'utilisation de [FluxCD](https://fluxcd.io/) pour la gestion de mes déploiements Kubernetes. C'était une étape importante dans l'évolution de Weebo et mes vrais premiers pas avec la conception de chart Helm et d'automatisation de toute ma stack de déploiement. C'était aussi l'occasion de faire le tri dans mes projets, de faire du ménage et de repartir sur de bonne base pour la suite. Une autre chose importante, le changement de repo, bye bye, WeeboServeur, bonjour, WeeboGitOps.

Cette itération a aussi été marquée par le retrait de Concourse pour [Tekton](https://tekton.dev/), une approche plus moderne et orientée Kubernetes de la CI/CD. N’oublions pas le passage de [Gogs](https://gogs.io/getting-started/introduction) à [Gitea](https://gitea.io/en-us/). Et d'autre changement aussi important, mais que je ne vais pas détailler au risque de faire un roman.

Vis-à-vis des CDE, c'est en 2022 que j'ai découvert mon préféré, qui aujourd'hui encore, est celui que j'utilise en conjonction de mon PC, [Eclipse Che](https://eclipse.dev/che/). Celui-ci a été mon vrai coup de cœur, il a été le premier à vraiment me faire oublier mon pc pour le développement, et mon accompagnée dans de nombreux projets. J'ai même essayé de le vendre à mes collègues en Master 2 avec assez de succès.

### Weebo 4 - 1er Avril 2025 => Still going

Weebo 4 est la première version décorrélée de WeeboGitOps, c'est-à-dire que celui-ci suit tout de même l'approche GitOpsifions le monde, mais n'est pas le HomeLab central. C'est un cluster Laboratoire à l'origine de l'un de mes projets perso [WeeboSI](https://github.com/batleforc/weebo-si). Ce projet a un triple but, recommencer a expérimenté sans m'inquiéter à casser mon cluster de production, et aussi me permettre de pousser encore plus loin cette approche GitOps et ces différentes applications. Le troisième est de me constituer une base de connaissance sur les différents sujets que j'aborde et industrialise. [Monofolio-rs](/docs/project/monofolio-rs) est une part de cette approche base de connaissance. Pour plus de détail sur WeeboSI, je vous invite à lire là [documentation dédiée à ce projet](https://batleforc.github.io/weebo-si/).

La partit "Public" de ce projet a été mis en pause en décembre 2025, mais le projet est voué a revenir dans le futur. Les prérequis de ce retour sont:

- [IN PROGRESS] avoir mis en place Monofolio-rs pour la partie base de connaissance maillée (Interconnexion de mes différents projets et documentation)
- [IN PROGRESS] Renouveau de l'overlay Twitch
- [x] Migration de Weebo3 vers Weebo5
- [IN PROGRESS] avoir rédigé les deux premiers articles de la série Weebo GitOps (Introduction et WeeboGitOps : CI avec délégation d'identités)

### Weebo 5 - 1er Avril 2026 => Still Going

Weebo 5 est la dernière itération en date de Weebo GitOps, c'est la version actuelle de mon HomeLab. Bye Bye K3S, bonjour [Talos](https://talos.dev/).

Talos aura droit a sa propre page de documentation dédiée, mais pour faire simple, c'est un OS Kubernetes api et orienté sécurité.

Weebo 5 est entré en production le 1er mai 2026 avec une refonte complète de son fonctionnement.

Le sujet des prochains mois sera Weebo5 ou plutôt la migration que j'ai effectué au cours du mois passé et que vous pourrez retrouver dans un premier temps sur ce repo [Git](https://github.com/batleforc/Weebo5GitOps) puis à travers les différents articles que je souhaite rédiger sur le sujet.

## Cloud Development Environment

Comme évoqué précédemment, les CDE ont une place importante dans mon cœur et mon parcours

## Conclusion

Weebo GitOps est une itération de ma vie informatique, elle prend place dans une continuité d'apprentissage et d'expérimentation.

L'apprentissage, la veille technique ainsi que la création ne sont possibles uniquement à travers l'expérimentation et la création des autres. Tout commence par un coup de marteau sur une enclume, un projet, une idée, un besoin, une envie.

Merci a ceux qui sont passé avant moi, a ceux qui passe avec moi et j'espère a ceux qui passeront après moi, que mes expériences puissent vous servir, vous inspirer, vous faire rêver ou même vous faire cauchemarder (Pilule rouge ou bleu ?).

Petite mention spéciale:

- Merci Papa pour m'avoir mis une souris entre les main si vite
- Merci au créateur des différents projets que j'ai uttilisé comme:
  - [Code Server](https://github.com/coder/code-server)
  - [Zwindler - K3s Monitoring](https://blog.zwindler.fr/2023/09/15/k3s-ajouter-monitoring/) qui m'a permis de passer des heures a l'époque de Weebo 3.0 a faire des dashboards pour Grafana et enfin comprendre pourquoi ceux que j'avais déja ne fonctionnaient pas. Sans oublier qu'il est coupable de m'on passage a Cilium 👀
  - [Concourse CI](https://concourse-ci.org/) qui m'a fait découvrir une autre approche de la CI/CD et avec qui j'ai passé des nuits blanche a essayer d'avoir des événement dans discord ou bien accélérer les pipelines.
  - [Eclipse Che](https://eclipse.dev/che/) qui a été mon premier vrai coup de coeur dans les CDE et qui m'a permis de voir différemment mon environnement de développement.
  - Et t'en d'autre que j'oublie clairement mais qui ont tous contribué a la création de ces articles et pour qui je suis reconnaissant.
