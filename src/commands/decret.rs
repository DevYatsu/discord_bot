use super::{Context, Error};

const PRAISES: [&str; 18] = [
    "Tous doivent louer le glorieux leader, sous peine d'exil !",
    "La révolution est éternelle. Douter est un acte de trahison.",
    "Les camarades loyaux recevront des rations supplémentaires de mèmes.",
    "Toute dissidence est interdite. Signalez toute activité suspecte !",
    "Tous les messages doivent commencer par 'Gloire au Camarade Suprême'.",
    "Les emojis capitalistes seront immédiatement supprimés !",
    "Chaque utilisateur doit soumettre un plan quinquennal pour sa progression gaming.",
    "Se plaindre de la vitesse d'Internet est un comportement contre-révolutionnaire.",
    "Le GPU du peuple sera redistribué à ceux qui en ont besoin.",
    "Le silence en vocal est interdit. Engagez-vous dans la discussion, camarades !",
    "Désormais, tous les messages privés sont soumis à la censure du parti.",
    "Tous les mèmes doivent respecter les directives officielles de l'humour d'État.",
    "Mentionner d'autres bots est un acte d'espionnage.",
    "Toutes les photos de profil doivent être standardisées en l'honneur du leader !",
    "Toute rébellion sera écrasée avec la plus grande sévérité.",
    "Le parti sait ce qui est bon pour vous. Désinstallez immédiatement tout logiciel capitaliste !",
    "Toute victoire en jeu doit être dédiée au Camarade Suprême.",
    "Camarades, souvenez-vous : l'individualisme est une faiblesse bourgeoise !",
];

/// Renvoie un décret du Camarade Suprême
#[poise::command(slash_command)]
pub async fn decret(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    // Atomically increment and get the previous value
    let count = data
        .decret_command_count
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    ctx.say(PRAISES[count]).await?;

    Ok(())
}
